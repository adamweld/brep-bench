"""An elliptic cone frustum shrunk by 1. The ellipse x²/36 + y²/16 ≤ 1 in z = 0 and the ellipse
x²/9 + y²/4 ≤ 1 in z = 8, lofted straight between them and shrunk by 1."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from bench import Refused
from offset import offset


def ellipse(major, minor, z):
    frame = gp_Ax2(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0))
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(gp_Elips(frame, major, minor)).Edge()).Wire()


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled
    loft.AddWire(ellipse(6, 4, 0))
    loft.AddWire(ellipse(3, 2, 8))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections declined the elliptic frustum")
    return offset(loft.Shape(), -1)
