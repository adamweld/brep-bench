"""A cube with a smaller cube cut from its corner. A cube of side 10 from the origin to (10, 10, 10)
less a cube of side 4 from the origin to (4, 4, 4)."""
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
    big = box(0, 0, 0, 10, 10, 10)
    small = box(0, 0, 0, 4, 4, 4)
    return run(BRepAlgoAPI_Cut(big, small), "BRepAlgoAPI_Cut")
