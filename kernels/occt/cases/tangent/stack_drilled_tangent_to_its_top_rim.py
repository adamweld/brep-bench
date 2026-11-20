"""A stack drilled through, the drill's wall touching the cone's top rim. The cylinder of radius 10 from z = 0 to z = 4,
with on it the cone frustum from z = 4 (radius 6) to z = 10 (radius 2), less the cylinder of radius 1 about the vertical line
through (1, 0)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    disc = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 10, 4).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 4), gp.DZ_s()), 6, 2, 6).Shape()
    fuse = BRepAlgoAPI_Fuse(disc, frustum)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the disc and the frustum")
    # the drill runs from z = -1 to z = 11
    drill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(1, 0, -1), gp.DZ_s()), 1, 12).Shape()
    cut = BRepAlgoAPI_Cut(fuse.Shape(), drill)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the stack and the drill")
    return cut.Shape()
