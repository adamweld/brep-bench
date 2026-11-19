"""A hole spotfaced flat into a sloping face. A block with a sloping top, and a vertical hole through
it whose seat is spotfaced flat into the slope, so that a bolt's head bears square to it.

The block: the polygon through (x, z) = (0, 0), (50, 0), (50, 20), (0, 30) in the plane y = 0,
extruded along +y to y = 30. Its top is a slope falling from z = 30 at x = 0 to z = 20 at x = 50,
the plane z = 30 − x/5.

Both features are about the vertical axis through (25, 15):

- the spotface: radius 8, cut down from the slope to a flat, level floor at z = 22. The slope
  crosses its wall between z = 23.4 (at x = 33) and z = 26.6 (at x = 17), so the wall is everywhere
  above the floor, and its rim in the slope is an ellipse;
- the hole: radius 4, from the spotface's floor right through the block to the bottom face (z = 0).

If you cut the hole with a cylinder, make it run past the bottom face."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused
from hole import cutter

SECTION_XZ = [(0, 0), (50, 0), (50, 20), (0, 30)]
# The half section from z = 30, above the slope, down: the spotface, radius 8, to its floor at
# z = 22, then the hole, radius 4, 1 past the bottom face.
HOLE = [(0, -1), (8, -1), (8, 8), (4, 8), (4, 31), (0, 31)]


def build():
    outline = BRepBuilderAPI_MakePolygon()
    for x, z in SECTION_XZ:
        outline.Add(gp_Pnt(x, 0, z))
    outline.Close()
    block = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(outline.Wire()).Face(),
                                  gp_Vec(0, 30, 0)).Shape()
    cut = BRepAlgoAPI_Cut(block, cutter((25, 15, 30), (0, 0, -1), HOLE))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the spotfaced hole")
    return cut.Shape()
