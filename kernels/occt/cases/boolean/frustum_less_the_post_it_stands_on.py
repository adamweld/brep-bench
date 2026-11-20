"""A frustum less the post it stands on. A post of radius 5 from z = 0 to z = 6. A frustum on its top,
from z = 6 to z = 10, radius 3 at the bottom and 1 at the top. Cut the post away from the frustum."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 6).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 6), gp.DZ_s()), 3, 1, 4).Shape()
    cut = BRepAlgoAPI_Cut(frustum, post)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the frustum and the post")
    return cut.Shape()
