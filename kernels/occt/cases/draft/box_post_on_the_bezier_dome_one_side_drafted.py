"""A square post on the Bézier dome block, one side drafted. The block of nurbs/bezier-dome united
with the box [6, 12] × [6, 12] × [0, 26]; the face x = 12 drafted by 4°, pull +z, neutral plane
z = 0, so that it leans toward the post's inside as x = 12 − z·tan 4°."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import face_through, other_case
from draft import drafted, united


def build():
    block = other_case("nurbs/bezier-dome").build()
    post = BRepPrimAPI_MakeBox(gp_Pnt(6, 6, 0), gp_Pnt(12, 12, 26)).Shape()
    part = united(block, post)
    side = face_through(part, (12, 9, 24))
    return drafted(part, [side], 4, neutral=(0, 0, 0))
