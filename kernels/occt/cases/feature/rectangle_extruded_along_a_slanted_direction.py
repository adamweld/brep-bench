"""A rectangle extruded along a slanted direction. The rectangle from (-10, -6, 0) to (10, 6, 0),
extruded by the vector (6, 0, 8)."""
from edit import extruded


def build():
    return extruded([(-10, -6, 0), (10, -6, 0), (10, 6, 0), (-10, 6, 0)], (6, 0, 8))
