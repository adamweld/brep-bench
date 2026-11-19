"""A half cone cut by a thin cylinder across its slanted wall. The half with y >= 0 of the cone of base
radius 10 and height 20, less the cylinder of radius 1 about the vertical line through (4, 3)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def box(x0, y0, z0, x1, y1, z1):
    return BRepPrimAPI_MakeBox(gp_Pnt(x0, y0, z0), gp_Pnt(x1, y1, z1)).Shape()


def run(op, name):
    if not op.IsDone():
        raise Refused(f"{name} declined")
    return op.Shape()


import math


def build():
    cone = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 10, 0, 20, math.pi).Shape()
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(4, 3, -5), gp.DZ_s()), 1, 30).Shape()
    return run(BRepAlgoAPI_Cut(cone, drill), "BRepAlgoAPI_Cut")
