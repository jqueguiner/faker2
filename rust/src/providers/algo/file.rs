//! Algorithmic formatters for the `file` provider.
#![allow(unused_variables, clippy::all)]
use crate::faker::Faker;

// ── MIME types ────────────────────────────────────────────────────────────────

const APPLICATION_MIME: &[&str] = &[
    "application/atom+xml",
    "application/ecmascript",
    "application/EDI-X12",
    "application/EDIFACT",
    "application/json",
    "application/javascript",
    "application/octet-stream",
    "application/ogg",
    "application/pdf",
    "application/postscript",
    "application/rdf+xml",
    "application/rss+xml",
    "application/soap+xml",
    "application/font-woff",
    "application/xhtml+xml",
    "application/xml-dtd",
    "application/xop+xml",
    "application/zip",
    "application/gzip",
];

const AUDIO_MIME: &[&str] = &[
    "audio/basic",
    "audio/L24",
    "audio/mp4",
    "audio/mpeg",
    "audio/ogg",
    "audio/vorbis",
    "audio/vnd.rn-realaudio",
    "audio/vnd.wave",
    "audio/webm",
];

const IMAGE_MIME: &[&str] = &[
    "image/gif",
    "image/jpeg",
    "image/pjpeg",
    "image/png",
    "image/svg+xml",
    "image/tiff",
    "image/vnd.microsoft.icon",
];

const MESSAGE_MIME: &[&str] = &[
    "message/http",
    "message/imdn+xml",
    "message/partial",
    "message/rfc822",
];

const MODEL_MIME: &[&str] = &[
    "model/example",
    "model/iges",
    "model/mesh",
    "model/vrml",
    "model/x3d+binary",
    "model/x3d+vrml",
    "model/x3d+xml",
];

const MULTIPART_MIME: &[&str] = &[
    "multipart/mixed",
    "multipart/alternative",
    "multipart/related",
    "multipart/form-data",
    "multipart/signed",
    "multipart/encrypted",
];

const TEXT_MIME: &[&str] = &[
    "text/cmd",
    "text/css",
    "text/csv",
    "text/html",
    "text/javascript",
    "text/plain",
    "text/vcard",
    "text/xml",
];

const VIDEO_MIME: &[&str] = &[
    "video/mpeg",
    "video/mp4",
    "video/ogg",
    "video/quicktime",
    "video/webm",
    "video/x-matroska",
    "video/x-ms-wmv",
    "video/x-flv",
];

const MIME_CATEGORIES: &[&[&str]] = &[
    APPLICATION_MIME,
    AUDIO_MIME,
    IMAGE_MIME,
    MESSAGE_MIME,
    MODEL_MIME,
    MULTIPART_MIME,
    TEXT_MIME,
    VIDEO_MIME,
];

// ── File extensions ───────────────────────────────────────────────────────────

const AUDIO_EXT: &[&str] = &["flac", "mp3", "wav"];

const IMAGE_EXT: &[&str] = &["bmp", "gif", "jpeg", "jpg", "png", "tiff"];

const TEXT_EXT: &[&str] = &["css", "csv", "html", "js", "json", "txt"];

const VIDEO_EXT: &[&str] = &["mp4", "avi", "mov", "webm"];

const OFFICE_EXT: &[&str] = &[
    "doc", "docx", "xls", "xlsx", "ppt", "pptx", "odt", "ods", "odp", "pages", "numbers", "key",
    "pdf",
];

const EXT_CATEGORIES: &[&[&str]] = &[AUDIO_EXT, IMAGE_EXT, OFFICE_EXT, TEXT_EXT, VIDEO_EXT];

const UNIX_DEVICE_PREFIXES: &[&str] = &["sd", "vd", "xvd"];

// ── helper functions ──────────────────────────────────────────────────────────

fn mime_type(f: &Faker) -> String {
    let cat = MIME_CATEGORIES[f.rng.below(MIME_CATEGORIES.len())];
    cat[f.rng.below(cat.len())].to_string()
}

fn file_extension_any(f: &Faker) -> String {
    let cat = EXT_CATEGORIES[f.rng.below(EXT_CATEGORIES.len())];
    cat[f.rng.below(cat.len())].to_string()
}

#[allow(dead_code)]
fn file_extension_for(f: &Faker, category: &str) -> String {
    let cat: &[&str] = match category {
        "audio" => AUDIO_EXT,
        "image" => IMAGE_EXT,
        "text" => TEXT_EXT,
        "video" => VIDEO_EXT,
        "office" => OFFICE_EXT,
        _ => EXT_CATEGORIES[f.rng.below(EXT_CATEGORIES.len())],
    };
    cat[f.rng.below(cat.len())].to_string()
}

fn file_name(f: &Faker, _locale: &str) -> String {
    let word = f.word();
    let ext = file_extension_any(f);
    format!("{word}.{ext}")
}

fn file_extension(f: &Faker, _locale: &str) -> String {
    file_extension_any(f)
}

fn file_path(f: &Faker, locale: &str) -> String {
    // Default: depth=1, absolute=true, linux
    let word = f.word();
    let ext = file_extension_any(f);
    let dir = f.word();
    format!("/{dir}/{word}.{ext}")
}

fn unix_device(f: &Faker, _locale: &str) -> String {
    let prefix = UNIX_DEVICE_PREFIXES[f.rng.below(UNIX_DEVICE_PREFIXES.len())];
    let letter = f.rng.random_lowercase_letter();
    format!("/dev/{prefix}{letter}")
}

fn unix_partition(f: &Faker, locale: &str) -> String {
    let device = unix_device(f, locale);
    let digit = f.rng.random_digit();
    format!("{device}{digit}")
}

/// Return `Some(value)` for a formatter this module implements, else `None`.
pub fn dispatch(f: &Faker, locale: &str, name: &str) -> Option<String> {
    Some(match name {
        "mime_type" => mime_type(f),
        "file_name" => file_name(f, locale),
        "file_extension" => file_extension(f, locale),
        "file_path" => file_path(f, locale),
        "unix_device" => unix_device(f, locale),
        "unix_partition" => unix_partition(f, locale),
        _ => return None,
    })
}
