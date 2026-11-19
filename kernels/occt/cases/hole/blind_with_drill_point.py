"""A blind drilled hole ending in a 118° drill point. A block, the box from the origin to (30, 30,
25), with a blind hole drilled down into it from the top face, its axis the vertical line through
(15, 15).

The hole has radius 4. Its wall runs down from the top face (z = 25) at its full radius for 12, to
z = 13. There it ends in the point a twist drill leaves: a cone about the axis with an included
angle of 118° (its side at 59° to the axis), its base the circle of radius 4 at z = 13 and its apex
on the axis below, at z = 13 − 4 cot 59° ≈ 10.597. The hole does not reach the bottom face."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from bench import Refused
from hole import cutter

RADIUS, DEPTH, POINT_ANGLE = 4, 12, 118

# The drill's half section from the top face down: its full radius to the depth, then its point.
POINT = RADIUS / math.tan(math.radians(POINT_ANGLE / 2))
SECTION = [(0, -1), (RADIUS, -1), (RADIUS, DEPTH), (0, DEPTH + POINT)]


def build():
    block = BRepPrimAPI_MakeBox(30, 30, 25).Shape()
    cut = BRepAlgoAPI_Cut(block, cutter((15, 15, 25), (0, 0, -1), SECTION))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the drilled hole")
    return cut.Shape()
