"""A recessed plate with a grid of pins. A plate from (0, 0, 0) to (40, 30, 4) with a recess from (3, 3, 2) to
(37, 27, 4), and twelve pins of radius 1 and height 1.5 on its floor, at x = 8, 16, 24, 32 and y = 9, 15, 21."""
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
    plate = box(0, 0, 0, 40, 30, 4)
    shape = run(BRepAlgoAPI_Cut(plate, box(3, 3, 2, 37, 27, 4)), "BRepAlgoAPI_Cut")
    for x in (8, 16, 24, 32):
        for y in (9, 15, 21):
            pin = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(x, y, 2), gp.DZ_s()), 1, 1.5).Shape()
            shape = run(BRepAlgoAPI_Fuse(shape, pin), "BRepAlgoAPI_Fuse")
    return shape
