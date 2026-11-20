"""A square post on a torus, one side drafted. The torus about the z axis, its tube of radius 5
centered on the circle of radius 15 in the plane z = 0, united with the box
[11, 19] × [−4, 4] × [0, 14]; the face x = 19 drafted by 4°, pull direction +z, neutral plane z = 0,
so that it leans toward the post's inside as x = 19 − z·tan 4°."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeTorus
from OCP.gp import gp_Pnt

from bench import face_through
from draft import drafted, united


def build():
    torus = BRepPrimAPI_MakeTorus(15, 5).Shape()
    post = BRepPrimAPI_MakeBox(gp_Pnt(11, -4, 0), gp_Pnt(19, 4, 14)).Shape()
    part = united(torus, post)
    side = face_through(part, (19, 0, 10))
    return drafted(part, [side], 4, neutral=(0, 0, 0))
