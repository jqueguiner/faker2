"""Sweep balanced-homophone weights per country/language.

No labelled homophone set exists. To avoid a circular objective (the dataset's
IPA is a single English G2P, so defining truth via IPA just maximises the IPA
weight), we build pseudo ground truth from SPELLING + METAPHONE only, leaving
IPA as an independent signal the sweep can genuinely evaluate per language:

  * POSITIVE  = metaphone-match AND spelling-close (<=1 edit)
                -> near-identical name, almost certainly a real variant.
  * NEGATIVE  = metaphone-match AND spelling-far (>=4 edits)
                -> a coarse collision (the "Sophie/Xavier" case).

For each country we grid-search (w_ipa, min_threshold, cap) to maximise F1 at
separating POSITIVE from NEGATIVE, then write per-country params to
data/balanced_params.json (+ a global default). Languages with regular
orthography settle on higher spelling weight; irregular ones on higher IPA.

Run:  PYTHONPATH=. python3 scripts/sweep_balanced.py [--probes N] [--work M] [--min-names K]
"""

import argparse
import json

import g2p2

from faker2.naming import realnames as rn


def g2p_sim(a, b):
    """Per-language phonetic similarity of two g2p IPA strings (0..1)."""
    if not a or not b:
        return 0.0
    return g2p2.similarity(a, b, "weighted")


W_IPA_GRID = [0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8]
MIN_GRID = [0.40, 0.45, 0.50, 0.55, 0.60]
CAP_GRID = [1, 2]
META_BONUS = 0.05


def lev(a, b, cap):
    return rn._levenshtein(a, b, cap)


def sim(a, b):
    if not a or not b:
        return 0.0
    m = max(len(a), len(b))
    return 1.0 - lev(a, b, m) / m


def build_probes(table, probes, work):
    """Return (probe_rows, working_rows) sorted by share desc."""
    rows = [(nm, r) for nm, r in table.items() if r[1]]  # need IPA
    rows.sort(key=lambda kv: kv[1][2], reverse=True)
    working = rows[:work]
    probe_rows = [(nm, r) for nm, r in rows[:probes]]
    return probe_rows, working


def candidates_for(probe, working):
    """Precompute per-candidate signals + P/N membership for one probe."""
    pnm, (p_ascii, p_ipa, _p_share, p_phon) = probe
    out = []
    for cnm, (c_ascii, c_ipa, c_share, c_phon) in working:
        if cnm == pnm:
            continue
        idist = lev(p_ipa, c_ipa, 4) if c_ipa else 99
        sdist = lev(p_ascii, c_ascii, 5)
        meta = 1.0 if (p_phon and c_phon == p_phon) else 0.0
        # truth from spelling+metaphone only (IPA independent)
        in_p = meta == 1.0 and sdist <= 1
        in_n = meta == 1.0 and sdist >= 4
        # keep only pool-relevant candidates to bound work
        if not (in_p or in_n or idist <= 2 or meta):
            continue
        out.append(
            {
                "ipa_sim": g2p_sim(p_ipa, c_ipa),
                "spell_sim": sim(p_ascii, c_ascii),
                "meta": meta,
                "idist": idist,
                "P": in_p,
                "N": in_n,
            }
        )
    return out


def score_country(table, probes, work):
    probe_rows, working = build_probes(table, probes, work)
    prepared = [candidates_for(p, working) for p in probe_rows]
    # drop probes with no usable P/N signal
    prepared = [c for c in prepared if any(x["P"] or x["N"] for x in c)]
    if not prepared:
        return None
    best = None
    for w_ipa in W_IPA_GRID:
        w_spell = 1.0 - w_ipa
        for cap in CAP_GRID:
            for mn in MIN_GRID:
                tp = fp = fn = 0
                for cands in prepared:
                    for c in cands:
                        in_pool = c["idist"] <= cap or c["meta"] == 1.0
                        if not in_pool:
                            if c["P"]:
                                fn += 1
                            continue
                        score = w_ipa * c["ipa_sim"] + w_spell * c["spell_sim"] + META_BONUS * c["meta"]
                        sel = score >= mn
                        if sel and c["P"]:
                            tp += 1
                        elif sel and c["N"]:
                            fp += 1
                        elif not sel and c["P"]:
                            fn += 1
                prec = tp / (tp + fp) if (tp + fp) else 0.0
                rec = tp / (tp + fn) if (tp + fn) else 0.0
                f1 = 2 * prec * rec / (prec + rec) if (prec + rec) else 0.0
                key = (f1, w_ipa, cap, -mn)
                if best is None or key > best[0]:
                    best = (
                        key,
                        {
                            "w_ipa": w_ipa,
                            "min": mn,
                            "cap": cap,
                            "f1": round(f1, 3),
                            "tp": tp,
                            "fp": fp,
                            "fn": fn,
                        },
                    )
    return best[1] if best else None


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--probes", type=int, default=80)
    ap.add_argument("--work", type=int, default=3000)
    ap.add_argument("--min-names", type=int, default=300)
    ap.add_argument("--out", default="data/balanced_params.json")
    args = ap.parse_args()

    bank = rn._bank()
    countries = sorted(bank._by_country, key=lambda c: -len(bank._by_country[c]))
    result = {}
    for cc in countries:
        table = bank._by_country[cc]
        if len(table) < args.min_names:
            continue
        r = score_country(table, args.probes, args.work)
        if r:
            result[cc] = r
            print(
                f"{cc}  w_ipa={r['w_ipa']} min={r['min']} cap={r['cap']} "
                f"F1={r['f1']} (tp={r['tp']} fp={r['fp']} fn={r['fn']})"
            )

    # global default = averaged best over countries (rounded to grid)
    import statistics as st

    def nearest(v, grid):
        return min(grid, key=lambda g: abs(g - v))

    default = {
        "w_ipa": nearest(st.mean(r["w_ipa"] for r in result.values()), W_IPA_GRID),
        "min": nearest(st.mean(r["min"] for r in result.values()), MIN_GRID),
        "cap": round(st.mean(r["cap"] for r in result.values())),
    }
    payload = {
        "_default": default,
        "countries": {c: {k: r[k] for k in ("w_ipa", "min", "cap")} for c, r in result.items()},
    }
    with open(args.out, "w") as f:
        json.dump(payload, f, indent=2, sort_keys=True)
    print(f"\ncountries tuned: {len(result)}  default: {default}")
    print(f"written: {args.out}")


if __name__ == "__main__":
    main()
