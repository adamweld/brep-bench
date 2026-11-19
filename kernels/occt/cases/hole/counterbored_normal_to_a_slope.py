"""A counterbored hole square to a sloping face, coming out aslant. A wedge-shaped block with a
counterbored hole drilled square to its sloping face.

The block: the polygon through (x, z) = (0, 0), (60, 0), (60, 15), (0, 60) in the plane y = 0,
extruded along +y to y = 40. Its top is a slope falling from z = 60 at x = 0 to z = 15 at x = 60,
the plane 0.6x + 0.8z = 48, whose outward normal is (0.6, 0, 0.8).

The hole: its axis is the line square to the slope through the point (45, 20, 26.25) on it, running
into the block along −(0.6, 0, 0.8).

- The counterbore: radius 7 and 5 deep, measured along the axis from the slope, with a flat floor
  square to the axis.
- The hole: radius 4, from the counterbore's floor on along the axis right through the block. It
  comes out of the bottom face (z = 0) aslant, the axis meeting it at (25.3125, 20, 0), and its
  mouth there is an ellipse with semi-axes 5 along x and 4 along y.

If you cut the hole with a cylinder, make it run past the bottom face."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused
from hole import cutter

SECTION_XZ = [(0, 0), (60, 0), (60, 15), (0, 60)]
# The half section from the slope inward: the counterbore, radius 7 to depth 5, then the hole,
# radius 4, on to depth 40, past the bottom face everywhere round it.
HOLE = [(0, -1), (7, -1), (7, 5), (4, 5), (4, 40), (0, 40)]


def build():
    outline = BRepBuilderAPI_MakePolygon()
    for x, z in SECTION_XZ:
        outline.Add(gp_Pnt(x, 0, z))
    outline.Close()
    wedge = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(outline.Wire()).Face(),
                                  gp_Vec(0, 40, 0)).Shape()
    # Square to the slope, whose outward normal is (0.6, 0, 0.8): into the block along its reverse.
    cut = BRepAlgoAPI_Cut(wedge, cutter((45, 20, 26.25), (-0.6, 0, -0.8), HOLE))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the counterbored hole")
    return cut.Shape()
