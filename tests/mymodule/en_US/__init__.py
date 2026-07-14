from faker2.providers import BaseProvider


class Provider(BaseProvider):
    def foo(self):
        return "bar"
