"""Two balls: one of radius 6 centred on the origin and one of radius 2 centred at (3, 6, 2), which is 7 from the origin, a little less than the 8 the radii add up to. They overlap in a small lens, but neither holds the other. Make the union.

The line between their centres is off every axis, so the circle where their spheres meet lies in a plane aslant to x, y and z, and cuts only a shallow cap from the big ball.
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    big = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 6).Shape()
    small = BRepPrimAPI_MakeSphere(gp_Pnt(3, 6, 2), 2).Shape()
    result = BRepAlgoAPI_Fuse(big, small)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two balls")
    return result.Shape()
