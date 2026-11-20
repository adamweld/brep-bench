"""A square post on a dome, one side drafted. The hemisphere of radius 20 (z ≥ 0) united with the
box [−5, 5] × [−5, 5] × [0, 30]; the face x = 5 drafted by 4°, pull direction +z, neutral plane
z = 0, so that it leans toward the post's inside as x = 5 − z·tan 4°."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import face_through
from draft import drafted, united


def build():
    dome = BRepPrimAPI_MakeSphere(20, 0, math.pi / 2).Shape()
    post = BRepPrimAPI_MakeBox(gp_Pnt(-5, -5, 0), gp_Pnt(5, 5, 30)).Shape()
    part = united(dome, post)
    side = face_through(part, (5, 0, 25))
    return drafted(part, [side], 4, neutral=(0, 0, 0))
