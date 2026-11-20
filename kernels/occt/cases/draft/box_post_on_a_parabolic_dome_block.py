"""A square post on a block under a parabolic dome, one side drafted. The solid turned about the z
axis from the region under the quadratic Bézier arc from (ρ, z) = (0, 10) through (10, 10) to
(20, 4), above z = 0, united with the box [2, 10] × [−4, 4] × [0, 30]; the face x = 10 drafted by
4°, pull +z, neutral plane z = 0, so that it leans toward the post's inside as x = 10 − z·tan 4°."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import face_through
from draft import drafted, revolved_bezier_block, united


def build():
    block = revolved_bezier_block([(0, 10), (10, 10), (20, 4)], 20)
    post = BRepPrimAPI_MakeBox(gp_Pnt(2, -4, 0), gp_Pnt(10, 4, 30)).Shape()
    part = united(block, post)
    side = face_through(part, (10, 0, 25))
    return drafted(part, [side], 4, neutral=(0, 0, 0))
