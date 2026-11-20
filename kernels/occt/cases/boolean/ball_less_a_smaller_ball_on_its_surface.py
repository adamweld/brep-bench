"""A ball of radius 6 about the origin, and a smaller ball of radius 5/2 centred at (2, 4, 4), which is 6 from the origin, on the big ball's surface. Cut the small ball from the big one.

The small ball bites a dent into the big ball's surface, bounded by a cap of the small ball's surface; the circle where the two spheres meet lies in a plane aslant to x, y and z. 
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    big = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 6).Shape()
    small = BRepPrimAPI_MakeSphere(gp_Pnt(2, 4, 4), 2.5).Shape()
    result = BRepAlgoAPI_Cut(big, small)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two balls")
    return result.Shape()
