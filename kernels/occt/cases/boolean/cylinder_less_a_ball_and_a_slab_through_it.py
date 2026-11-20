"""A cylinder less a ball and a slab through it. A cylinder of radius 3 about the z axis from z = 0
to z = 20, fused with the slab (−6, −6, 9) to (6, 6, 11); a ball of radius 5 centred at (0, 0, 10),
fused with the same slab; the second cut from the first."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def fused(a, b):
    fuse = BRepAlgoAPI_Fuse(a, b)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse failed")
    return fuse.Shape()


def build():
    slab = BRepPrimAPI_MakeBox(gp_Pnt(-6, -6, 9), gp_Pnt(6, 6, 11)).Shape()
    first = fused(BRepPrimAPI_MakeCylinder(3, 20).Shape(), slab)
    second = fused(BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 10), 5).Shape(), slab)
    cut = BRepAlgoAPI_Cut(first, second)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut failed")
    return cut.Shape()
