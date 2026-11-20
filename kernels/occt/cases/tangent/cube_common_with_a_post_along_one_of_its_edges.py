"""A cube and a post along one of its edges, their common part. A cube with edge 10 from the origin, common with a cylinder of radius 5, its axis the z axis, from z = -5 to 10."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -5), gp.DZ_s()), 5, 15).Shape()
    common = BRepAlgoAPI_Common(cube, post)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and the post")
    return common.Shape()
