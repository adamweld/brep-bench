"""A ball-tipped cone shrunk past its tip's radius. A cone with its tip rounded: the cone on the z
axis with its base, of radius 6, at z = 0 and its apex at z = 12, so that its side leans in 1 for
every 2 it rises, and the tip rounded by a ball of radius 1 tangent to the side. The ball's
center is on the axis at z = 12 − √5, and it touches the side along the circle of radius 2/√5 at
z = 12 − 4/√5; above that circle the solid's surface is the ball, below it the cone. Shrink it by
1.5: the solid of every point at least 1.5 from the solid's outside.

The side, moved 1.5 inward, is the cone about the same axis with the same lean and its apex 1.5√5
lower, at z = 12 − 1.5√5. The ball's face, moved 1.5 inward, would be a ball of radius 1 − 1.5, less
than nothing: it is no part of the result, and the moved side runs on to its own apex. The base
moves up to z = 1.5.

The result is a plain cone: its base, of radius h/2, at z = 1.5 and its apex h above, at z = 12 −
1.5√5, with h = 10.5 − 1.5√5 ≈ 7.146."""
import math

from offset import upright_cone


def build():
    # OCCT's offset keeps a face where the tip was; the stated cone is built outright.
    height = 12 - 1.5 * math.sqrt(5) - 1.5
    return upright_cone(1.5, height)
