"""A countersunk hole through a block, 90° included. A block, the box from the origin to (40, 30, 12),
with a countersunk hole through it, its axis the vertical line through (20, 15).

- The hole: radius 3, right through the block from the top (z = 12) to the bottom (z = 0).
- The countersink: a cone about the same axis with an included angle of 90° (its side at 45° to
  the axis), opening into the top face in a circle of radius 6. It narrows going down until it
  meets the hole's wall in the circle of radius 3 at z = 9. So the hole's wall runs from z = 0 to
  z = 9.

If you cut the hole with a cylinder, make it run past the bottom face."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from bench import Refused
from hole import cutter

# The half section from the top face down: the countersink at 45° to the axis, from radius 7 one
# above the face (6 at the face) to radius 3 at depth 3, then the hole through, 1 past the bottom.
SECTION = [(0, -1), (7, -1), (3, 3), (3, 13), (0, 13)]


def build():
    block = BRepPrimAPI_MakeBox(40, 30, 12).Shape()
    cut = BRepAlgoAPI_Cut(block, cutter((20, 15, 12), (0, 0, -1), SECTION))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the countersunk hole")
    return cut.Shape()
