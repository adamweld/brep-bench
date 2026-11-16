"""A small cube and itself turned 1.2° about one of its edges, their common part. A small cube, of
side 0.001, from the origin to (0.001, 0.001, 0.001), and a copy of itself turned 1.2° about the z
axis, the line of one of the cube's own edges: the part common to both.

The common part is a prism 0.001 high over the part common to the two squares, a quadrilateral with
corners (0, 0), s(cos θ, sin θ), (s(1 − sin θ)/cos θ, s) and (0, s): the copy's side through the
origin cuts a thin wedge off the cube, and its far side cuts off the corner opposite. Its area is
s²(1 − sin θ)/cos θ and its perimeter 2s + 2s(1 − sin θ)/cos θ. The turn moves the cube's far
corners by 0.000021 and 0.000030, small distances, but the common part is 2.1% less than the cube: a
kernel that takes the turned copy's sides for the cube's own, by a tolerance that does not shrink
with the model, returns the cube and is 2.1% out."""
from math import radians

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax1, gp_Pnt, gp_Trsf

from bench import Refused

SIDE = 0.001
TURN = radians(1.2)


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(SIDE, SIDE, SIDE)).Shape()
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, 0), gp.DZ_s()), TURN)
    turned = BRepBuilderAPI_Transform(cube, turn, True).Shape()
    result = BRepAlgoAPI_Common(cube, turned)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two cubes")
    return result.Shape()
