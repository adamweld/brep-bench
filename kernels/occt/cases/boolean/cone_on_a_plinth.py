"""A cone on a plinth. A disk of radius 6 from z = 0 to z = 2 with a cone standing on it, the
cone's base the disk of radius 4 in z = 2 and its apex at (0, 0, 10)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    plinth = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 6, 2).Shape()
    cone = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 2), gp.DZ_s()), 4, 0, 8).Shape()
    fuse = BRepAlgoAPI_Fuse(plinth, cone)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the cone and plinth")
    return fuse.Shape()
