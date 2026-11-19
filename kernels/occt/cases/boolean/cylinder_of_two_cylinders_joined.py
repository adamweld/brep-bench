"""A cylinder made of two cylinders joined end to end. Two cylinders of radius 5 and height 10 about the
z axis, one from z = 0 to 10 and one from z = 0 to −10, united."""
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


def build():
    up = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 10).Shape()
    down = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -10), gp.DZ_s()), 5, 10).Shape()
    return run(BRepAlgoAPI_Fuse(up, down), "BRepAlgoAPI_Fuse")
