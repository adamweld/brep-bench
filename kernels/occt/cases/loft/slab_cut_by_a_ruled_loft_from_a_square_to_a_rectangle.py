r"""A slab cut by a ruled loft from a square to a rectangle. A square slab cut through by a ruled loft from a square to a rectangle, leaving a hole whose four walls are flat and slanted.

The slab is the box −7 ≤ x, y ≤ 7, 0 ≤ z ≤ 3. The tool is a ruled loft between two horizontal rectangles centered on the z axis: the square −2 ≤ x, y ≤ 2 at z = −1, and the rectangle −1 ≤ x ≤ 1, −3 ≤ y ≤ 3 at z = 4. Corners are joined to the corners at the same side of both, (±2, ±2, −1) to (±1, ±3, 4) with the same signs, by straight lines, and each side of the tool is the ruled surface between the corresponding edges: the edge of the square at x = 2 (parallel to y) and the edge of the rectangle at x = 1 (also parallel to y) bound a flat quadrilateral, and likewise for the other three sides. So at height z the tool's section is the rectangle |x| ≤ 2 − u, |y| ≤ 2 + u, u = (z + 1)/5. The tool runs past the slab at both faces; the slab less the tool is the solid, a slab with a slanted hole through it, 3.6 × 4.4 at the slab's bottom face and 2.4 × 5.6 at its top, narrowing in x and widening in y as it rises.

**As a set:**

$$\{|x|, |y| \le 7,\ 0 \le z \le 3\} \setminus \{\,|x| < 2 - u,\ |y| < 2 + u\,\},\quad u = \tfrac{z + 1}{5}$$"""

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def rectangle(hx, hy, z):
    poly = BRepBuilderAPI_MakePolygon()
    for sx, sy in [(1, 1), (-1, 1), (-1, -1), (1, -1)]:
        poly.Add(gp_Pnt(sx * hx, sy * hy, z))
    poly.Close()
    return poly.Wire()


def build():
    slab = BRepPrimAPI_MakeBox(gp_Pnt(-7, -7, 0), gp_Pnt(7, 7, 3)).Shape()
    # a ruled solid loft: corner to the like corner, each side a flat quadrilateral
    loft = BRepOffsetAPI_ThruSections(True, True)
    loft.AddWire(rectangle(2, 2, -1))
    loft.AddWire(rectangle(1, 3, 4))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the square to the rectangle")
    cut = BRepAlgoAPI_Cut(slab, loft.Shape())
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the slab")
    return cut.Shape()
