"""A turned post on the Bézier dome block, its outer side drafted. The block of nurbs/bezier-dome
united with the rectangle from 2 to 5 out from the vertical line through (10, 10), z from 0 to 26,
in the plane y = 10, turned 150° counterclockwise about that line from +x; the outer face (radius 5
from the line) drafted by 4°, pull +z, neutral plane z = 0, into the cone of radius 5 − z·tan 4°."""
import math

from bench import face_through, other_case
from draft import drafted, turned_block, united


def build():
    block = other_case("nurbs/bezier-dome").build()
    post = turned_block((10, 10), 2, 5, 0, 26, 150)
    part = united(block, post)
    side = face_through(part, (10 + 5 * math.cos(math.radians(75)), 10 + 5 * math.sin(math.radians(75)), 24))
    return drafted(part, [side], 4, neutral=(0, 0, 0))
