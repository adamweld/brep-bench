"""A turned post on a sector of a frustum, its outer side drafted. The quarter (x ≥ 0, y ≥ 0) of the
frustum about the z axis, radius 20 at z = 0 to 12 at z = 20, united with the rectangle from 2 to 5
out from the vertical line through (10, 6), z from 0 to 26, in the plane y = 6, turned 150°
counterclockwise about that line from +x; the outer face (radius 5 from the line) drafted by 4°,
pull +z, neutral plane z = 0, into the cone of radius 5 − z·tan 4°."""
import math

from bench import face_through
from draft import drafted, frustum_sector, turned_block, united


def build():
    base = frustum_sector()
    post = turned_block((10, 6), 2, 5, 0, 26, 150)
    part = united(base, post)
    side = face_through(part, (10 + 5 * math.cos(math.radians(75)), 6 + 5 * math.sin(math.radians(75)), 24))
    return drafted(part, [side], 4, neutral=(0, 0, 0))
