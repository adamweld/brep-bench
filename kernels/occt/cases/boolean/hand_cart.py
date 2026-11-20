"""A hand cart: a chassis block, four wheel disks against its sides, a handle and a nose cone,
all fused one after another."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    solid = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 1.5), gp_Pnt(12, 6, 4.5)).Shape()
    parts = []
    for cx in (3, 9):
        for y0 in (-1, 6):
            parts.append(BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(cx, y0, 2), gp_Dir(0, 1, 0)), 2, 1).Shape())
    parts.append(BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(12, 3, 3), gp_Dir(1, 0, 0)), 0.5, 6).Shape())
    parts.append(BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 3, 3), gp_Dir(-1, 0, 0)), 1, 0, 3).Shape())
    for part in parts:
        fuse = BRepAlgoAPI_Fuse(solid, part)
        if not fuse.IsDone():
            raise Refused("BRepAlgoAPI_Fuse declined a part of the cart")
        solid = fuse.Shape()
    return solid
