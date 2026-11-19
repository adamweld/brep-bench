"""A cube notched by a cone that reaches two of its corners. The cube from (0, 0, 0) to
(10, 10, 10), less the solid cone frustum on the upright line x = 10, y = 5, of radius 5 at z = 0
and radius 2 at z = 10: its base circle touches the faces y = 0 and y = 10 at the cube's corners
(10, 0, 0) and (10, 10, 0)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    cone = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(10, 5, 0), gp.DZ_s()), 5, 2, 10).Shape()
    cut = BRepAlgoAPI_Cut(cube, cone)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cone")
    return cut.Shape()
