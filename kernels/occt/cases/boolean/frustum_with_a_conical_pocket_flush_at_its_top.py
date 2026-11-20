"""A frustum with a conical pocket flush at its top. A frustum on the z axis, radius 6 at z = 0 and 3
at z = 8; a narrower frustum from radius 1 at z = 4 to radius 2 at z = 8, its top disc in the plane
of the first's top. Cut the second from the first."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 6, 3, 8).Shape()
    pocket = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 4), gp.DZ_s()), 1, 2, 4).Shape()
    cut = BRepAlgoAPI_Cut(frustum, pocket)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the conical pocket")
    return cut.Shape()
