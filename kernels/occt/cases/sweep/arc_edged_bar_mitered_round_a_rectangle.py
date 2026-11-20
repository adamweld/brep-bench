r"""A quarter-round bar mitered round a rectangle. A bar with one rounded corner swept once round a rectangle, mitered at the four corners: a ring, a rectangle's border with a quarter-round edge.

The path: the rectangle in the xy plane with corners (±5, ±3, 0), 10 × 6, closed.

The section stands square to the path. It is given by s, the distance outward from the path (away from the rectangle's center), and z: the region 0 ≤ s ≤ 2, 0 ≤ z ≤ 1.5 with its corner at (s, z) = (2, 1.5) rounded off by a quarter of a circle of radius 1, centered on (s, z) = (1, 0.5). Its boundary is, in order: the line from (0, 0) to (2, 0); the line up to (2, 0.5); the arc of that circle, convex, from (2, 0.5) through (1 + 1/√2, 0.5 + 1/√2) to (1, 1.5); the line back to (0, 1.5); and the line down to (0, 0). That is a bar 2 wide and 1.5 tall lying outside the path, its outer top edge rounded. On the side y = −3, for example, the point (s, z) of the section is at (x, −3 − s, z).

Each side of the rectangle is the section swept straight along it, and at each corner the two sides are mitered: each is cut by, and they meet in, the vertical plane through the corner along its diagonal, at 45° to both sides. At (5, −3, 0) that is the plane x + y = 2, running from the path's corner out to the ring's outer corner at (7, −5). The path is closed, so the ring has no ends: it is 14 × 10 outside, with a plain rectangular opening 10 × 6.

**As a set:**

$$\{\,(x, y, z) : s = \max(|x| - 5,\ |y| - 3) \ge 0,\ 0 \le s \le 2,\ 0 \le z \le 1.5,\ \neg\,(s > 1 \wedge z > 0.5 \wedge (s - 1)^2 + (z - 0.5)^2 > 1)\,\}$$"""

import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp_Dir, gp_Pnt

from bench import Refused
from corner import MITRE, polyline, swept

W, H = 5, 3  # the path's rectangle, half-sides


def at(s, z):
    """The section's point (s, z) where the section stands, on the side y = −3."""
    return gp_Pnt(0, -H - s, z)


def section():
    k = 1 / math.sqrt(2)
    arc = GC_MakeArcOfCircle(at(2, 0.5), at(1 + k, 0.5 + k), at(1, 1.5)).Value()
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(at(0, 0), at(2, 0)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(at(2, 0), at(2, 0.5)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(arc).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(at(1, 1.5), at(0, 1.5)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(at(0, 1.5), at(0, 0)).Edge())
    if not wire.IsDone():
        raise Refused("BRepBuilderAPI_MakeWire could not close the section")
    return wire.Wire()


def build():
    # Round the rectangle anticlockwise from the middle of its side y = −3, where the section
    # stands, outward being −y there.
    path = polyline([(0, -H, 0), (W, -H, 0), (W, H, 0), (-W, H, 0), (-W, -H, 0)], closed=True)
    return swept(path, section(), MITRE, binormal=gp_Dir(0, 0, 1))  # the section kept upright
