"""A post less a cube beside its axis. A cube of side 10 from the origin to (10, 10, 10). A post of radius
4 and height 10, from z = 0 to z = 10, whose axis is the vertical line through (−2, 5). Cut the cube away
from the post."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-2, 5, 0), gp.DZ_s()), 4, 10).Shape()
    cut = BRepAlgoAPI_Cut(post, cube)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the post and the cube")
    return cut.Shape()
