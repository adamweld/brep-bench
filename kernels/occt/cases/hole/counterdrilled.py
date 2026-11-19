"""A counterdrilled hole: a wide bore stepping down to a narrow one by a 118° cone. A block, the box
from the origin to (30, 30, 20), with a counterdrilled hole through it, its axis the vertical line
through (15, 15). Seen from the top, going down:

- a bore of radius 5, at its full radius from the top face (z = 20) down to z = 12;
- then a cone about the axis with an included angle of 118° (its side at 59° to the axis), the
  point of the drill that made the bore, narrowing from radius 5 at z = 12 until it meets the
  narrow hole's wall at radius 2.5, at z = 12 − 2.5 cot 59° ≈ 10.498;
- then the narrow hole, radius 2.5, on down through the bottom face (z = 0).

If you cut the narrow hole with a cylinder, make it run past the bottom face."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from bench import Refused
from hole import cutter

WIDE, NARROW, DEPTH, POINT_ANGLE = 5, 2.5, 8, 118

# The half section from the top face down: the wide bore to its depth, its drill's point down to
# the narrow hole's radius, and the narrow hole on through, 1 past the bottom face.
STEP = (WIDE - NARROW) / math.tan(math.radians(POINT_ANGLE / 2))
SECTION = [(0, -1), (WIDE, -1), (WIDE, DEPTH), (NARROW, DEPTH + STEP), (NARROW, 21), (0, 21)]


def build():
    block = BRepPrimAPI_MakeBox(30, 30, 20).Shape()
    cut = BRepAlgoAPI_Cut(block, cutter((15, 15, 20), (0, 0, -1), SECTION))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the counterdrilled hole")
    return cut.Shape()
