"""A frustum lying across a post, less the post. A post of radius 4 on the z axis from z = −6 to z = 6.
A frustum on the x axis from x = −7 to x = 7, radius 2 at x = −7 and 1 at x = 7. Cut the post away from the
frustum."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -6), gp.DZ_s()), 4, 12).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(-7, 0, 0), gp_Dir(1, 0, 0)), 2, 1, 14).Shape()
    cut = BRepAlgoAPI_Cut(frustum, post)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the frustum and the post")
    return cut.Shape()
