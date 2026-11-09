"""Continuous torus tumble: the solid is defined in cases/swept/tumbling-torus-crossed-forward.md.

Uses the same unavailable operation as the baseline tumbling-torus case. A union
of finitely many poses would change the solid and is not a valid workaround.
"""
from bench import Refused


def build():
    raise Refused("occt has no continuous solid sweep under two turns about different axes; "
                  "its prism, revolution and pipe operations do not construct this torus envelope")
