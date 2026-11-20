"""A cube less a post along one of its edges. A cube of side 10 between the origin and (10, 10, 10),
less a solid cylinder of radius 5 on the line x = 0, y = 0, from z = −2 to z = 10, its top disc level
with the cube's top face."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused

SIDE = 10
POST_RADIUS = 5
POST_FROM, POST_TO = -2, 10  # z


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, POST_FROM), gp.DZ_s()), POST_RADIUS,
                                    POST_TO - POST_FROM).Shape()
    cut = BRepAlgoAPI_Cut(cube, post)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cube and the post")
    return cut.Shape()
