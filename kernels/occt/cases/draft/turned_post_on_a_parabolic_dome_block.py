"""A turned post on a block under a parabolic dome, its outer side drafted. The solid turned about the
z axis from the region under the quadratic Bézier arc from (ρ, z) = (0, 10) through (10, 10) to
(20, 4), above z = 0, united with the rectangle from 2 to 5 out from the vertical line through
(8, 0), z from 0 to 28, in the plane y = 0, turned 180° counterclockwise about that line from +x;
the outer face (radius 5 from the line) drafted by 4°, pull +z, neutral plane z = 0, into the cone of
radius 5 − z·tan 4°."""
from bench import face_through
from draft import drafted, revolved_bezier_block, turned_block, united


def build():
    block = revolved_bezier_block([(0, 10), (10, 10), (20, 4)], 20)
    post = turned_block((8, 0), 2, 5, 0, 28, 180)
    part = united(block, post)
    side = face_through(part, (8, 5, 25))
    return drafted(part, [side], 4, neutral=(0, 0, 0))
