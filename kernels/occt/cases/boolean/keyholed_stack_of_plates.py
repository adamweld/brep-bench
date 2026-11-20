"""A stack of plates: a square plate, a disk with a slot cut from its rim, a small disk on top."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def cyl(radius, z0, height):
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, z0), gp.DZ_s()), radius, height).Shape()


def op(kind, a, b, what):
    o = kind(a, b)
    if not o.IsDone():
        raise Refused(f"{kind.__name__} declined {what}")
    return o.Shape()


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, 0), gp_Pnt(10, 10, 1)).Shape()
    disk = cyl(6, 1, 2)
    # The slot's cutter runs past the disk's faces and its rim, but is cut before the stack is
    # joined, so that it does not reach the plate.
    slot = BRepPrimAPI_MakeBox(gp_Pnt(2, -1, 0), gp_Pnt(8, 1, 4)).Shape()
    disk = op(BRepAlgoAPI_Cut, disk, slot, "the slot")
    solid = op(BRepAlgoAPI_Fuse, plate, disk, "the disk")
    return op(BRepAlgoAPI_Fuse, solid, cyl(1.5, 3, 1), "the small disk")
