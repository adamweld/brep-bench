"""A small cylinder (radius 0.00075, height 0.0015) and a copy a hundredth wider
(radius 0.0007575) on the same axis: the wider copy less it."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

RADIUS = 0.00075
WIDER = 0.0007575
HEIGHT = 0.0015


def build():
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), WIDER, HEIGHT).Shape()
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), RADIUS, HEIGHT).Shape()
    result = BRepAlgoAPI_Cut(first, second)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two")
    return result.Shape()
