"""A drilled platform with a bar, common with itself turned a quarter turn. The plate (-30,-30,0)-(30,30,6)
with four holes of radius 4 at (+-22, +-22) and a bar (-20,-6,6)-(20,6,18), common with itself turned 90 degrees about z."""
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


from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.gp import gp_Trsf


def build():
    shape = box(-30, -30, 0, 30, 30, 6)
    for x in (-22, 22):
        for y in (-22, 22):
            drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(x, y, -1), gp.DZ_s()), 4, 8).Shape()
            shape = run(BRepAlgoAPI_Cut(shape, drill), "BRepAlgoAPI_Cut")
    shape = run(BRepAlgoAPI_Fuse(shape, box(-20, -6, 6, 20, 6, 18)), "BRepAlgoAPI_Fuse")
    turn = gp_Trsf()
    turn.SetRotation(gp.OZ_s(), math.pi / 2)
    turned = BRepBuilderAPI_Transform(shape, turn, True).Shape()
    return run(BRepAlgoAPI_Common(shape, turned), "BRepAlgoAPI_Common")
