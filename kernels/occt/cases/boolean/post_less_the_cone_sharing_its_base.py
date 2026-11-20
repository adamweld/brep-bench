"""A post less the cone that shares its base. A cone on the z axis: base radius 4 in z = 0, apex at
(0, 0, 8). A post of radius 4 from z = 0 to z = 4. Cut the cone away from the post."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cone = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 4, 0, 8).Shape()
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 4, 4).Shape()
    cut = BRepAlgoAPI_Cut(post, cone)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the post and the cone")
    return cut.Shape()
