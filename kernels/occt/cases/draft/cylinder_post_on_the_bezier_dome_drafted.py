"""A round post on the Bézier dome block, its side drafted. The block of nurbs/bezier-dome united
with the cylinder of radius 4 about the vertical line through (10, 10), z from 0 to 26; its side
drafted by 4°, pull +z, neutral plane z = 0, into the cone of radius 4 − z·tan 4°."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import face_through, other_case
from draft import drafted, united


def build():
    block = other_case("nurbs/bezier-dome").build()
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(10, 10, 0), gp_Dir(0, 0, 1)), 4, 26).Shape()
    part = united(block, post)
    side = face_through(part, (14, 10, 24))
    return drafted(part, [side], 4, neutral=(0, 0, 0))
