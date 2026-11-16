"""A ball-tipped cone shelled open at its base, the wall thicker than its tip's radius. A cone with
its tip rounded: the cone on the z axis with its base, of radius 6, at z = 0 and its apex at z =
12, so that its side leans in 1 for every 2 it rises, and the tip rounded by a ball of radius 1
tangent to the side. The ball's center is on the axis at z = 12 − √5, and it touches the side
along the circle of radius 2/√5 at z = 12 − 4/√5; above that circle the solid's surface is the
ball, below it the cone. Shell it to a wall 1.5 thick, open at its base: the base is removed, the
side and the tip move 1.5 inward, and the cavity they bound opens through the plane of the base.

The side, moved 1.5 inward, is the cone about the same axis with the same lean and its apex 1.5√5
lower, at z = 12 − 1.5√5. The ball's face, moved 1.5 inward, would be a ball of radius 1 − 1.5, less
than nothing: it is no part of the result, and the moved side runs on to its own apex. It runs down
to the plane z = 0, which it meets in the circle of radius H/2, H = 12 − 1.5√5 ≈ 8.646.

The result is a hollow cone standing on a flat ring: the ring from radius H/2 to radius 6 at z = 0,
the conical cavity above it with its apex at z = H, and the rounded cone outside."""
import math

from offset import ball_tipped_cone, less, upright_cone


def build():
    # OCCT's shell declines; the stated cavity is cut outright, its cone run on 1 below the base.
    height = 12 - 1.5 * math.sqrt(5)
    return less(ball_tipped_cone(), upright_cone(-1, height + 1), "the cavity")
