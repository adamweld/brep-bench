"""A hole bored by a drill pushed along its own axis. A plate, the box from (0, −5, 0) to (20, 5,
4), with a hole bored through it by a drill.

The drill is a solid of revolution about the vertical line through (10, 0): a cylinder of radius 2
from z = 6 + 2/tan 59° up to z = 14, ended below in a cone of 118° (each side 59° from the axis)
whose point is (10, 0, 6).

It slides without turning straight down along its axis by 20, from where its point is 2 above the
plate to where its top is 6 below it. Cut from the plate everything the drill passes through.

Every circle of the drill about its axis meets the motion at one angle all round, so it touches all
round or nowhere, and the circle of greatest radius sweeps the hole's wall. The drill passes wholly
through, so the hole is the cylinder of radius 2, whatever its point."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax1, gp_Pnt

from sweep import cut, slid

PLATE = ((0, -5, 0), (20, 5, 4))
AXIS = (10, 0)  # the drill's axis is the vertical line through this point
POINT, TOP = 6, 14  # heights of the drill's point and its flat top
RADIUS = 2
HALF_ANGLE = 59  # of the point, from the axis
PUSH = (0, 0, -20)


def build():
    x, y = AXIS
    shoulder = POINT + RADIUS / math.tan(math.radians(HALF_ANGLE))
    # the drill's half-section beside its axis, turned about the axis: a cone and a cylinder
    profile = [gp_Pnt(x, y, POINT), gp_Pnt(x + RADIUS, y, shoulder), gp_Pnt(x + RADIUS, y, TOP),
               gp_Pnt(x, y, TOP)]
    half = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakePolygon(*profile, True).Wire()).Face()
    drill = BRepPrimAPI_MakeRevol(half, gp_Ax1(gp_Pnt(x, y, 0), gp.DZ_s())).Shape()
    # Pushed along its axis, the drill's cylinder slides along itself, its cone leads all over
    # and its flat top trails: no face needs splitting.
    swept = slid(drill, PUSH)
    return cut(BRepPrimAPI_MakeBox(gp_Pnt(*PLATE[0]), gp_Pnt(*PLATE[1])).Shape(), swept)
