"""A cylinder of radius 4 and height 3 standing on the xy plane, its axis the z axis, and a ball of the same radius, 4, centred at (6, 0, 5), outside the cylinder: farther from its axis than its rim, and above its top. Cut the ball from the cylinder.

The ball reaches down to z = 1 and in to ρ = 2 along the x axis, so it bites a notch from the cylinder's top and rim near the point (4, 0, 3), its surface crossing the cylinder's top face and its wall.
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 4, 3).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(6, 0, 5), 4).Shape()
    result = BRepAlgoAPI_Cut(cylinder, ball)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cylinder and the ball")
    return result.Shape()
