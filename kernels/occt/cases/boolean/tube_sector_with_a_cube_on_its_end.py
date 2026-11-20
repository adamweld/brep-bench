"""A 120-degree sector of a thick tube, bored through axially, with a cube fused to its end."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def op(kind, a, b, what):
    o = kind(a, b)
    if not o.IsDone():
        raise Refused(f"{kind.__name__} declined {what}")
    return o.Shape()


def build():
    ax = gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s())
    angle = 2 * math.pi / 3
    outer = BRepPrimAPI_MakeCylinder(ax, 8, 6, angle).Shape()
    inner = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp.DZ_s()), 4, 8, angle).Shape()
    sector = op(BRepAlgoAPI_Cut, outer, inner, "the inner cylinder")
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(3, 3 * math.sqrt(3), -1), gp.DZ_s()), 1, 8).Shape()
    sector = op(BRepAlgoAPI_Cut, sector, bore, "the bore")
    cube = BRepPrimAPI_MakeBox(gp_Pnt(4.5, -3, 1.5), gp_Pnt(7.5, 0, 4.5)).Shape()
    return op(BRepAlgoAPI_Fuse, sector, cube, "the cube")
