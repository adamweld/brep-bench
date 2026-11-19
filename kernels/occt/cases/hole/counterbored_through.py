"""A counterbored hole through a block. A block, the box from the origin to (40, 30, 20), with a
counterbored hole through it, its axis the vertical line through (20, 15).

- The hole: radius 3, right through the block from the top (z = 20) to the bottom (z = 0).
- The counterbore: radius 5.5 and 6 deep, from the top face down to a flat floor at z = 14, square
  to the axis. So the hole's wall of radius 3 runs from z = 0 to z = 14, and the floor is the ring
  between radii 3 and 5.5 at z = 14.

If you cut the hole with a cylinder, make it run past the bottom face."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from bench import Refused
from hole import cutter

# The half section from the top face down, 1 above it and 1 past the bottom: the counterbore,
# radius 5.5 to depth 6, then the hole, radius 3, through the 20 of the block.
SECTION = [(0, -1), (5.5, -1), (5.5, 6), (3, 6), (3, 21), (0, 21)]


def build():
    block = BRepPrimAPI_MakeBox(40, 30, 20).Shape()
    cut = BRepAlgoAPI_Cut(block, cutter((20, 15, 20), (0, 0, -1), SECTION))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the counterbored hole")
    return cut.Shape()
