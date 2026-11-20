"""A square bar turned round a rectangular loop with round corners. A closed pipe of square section, swept along a closed path of four straight runs, the section
carried round each corner by turning it about the corner.

The path is the rectangle in the plane z = 0 with corners (0, 0, 0), (40, 0, 0), (40, 30, 0) and
(0, 30, 0), taken from (20, 0, 0) along +x, once round, and closed.

The section is the square of side 6, centered on the path and square to it, with two sides
parallel to the z axis: at (20, 0, 0) it has corners (20, 3, 3), (20, −3, 3), (20, −3, −3) and
(20, 3, −3). Each run is the square swept straight along it, ending flat in the plane through the
corner square to the run. At each corner the two runs overlap on the inside of the turn, and the
gap they leave outside is filled by the square turned about the vertical line through the corner,
a quarter turn from square to one run to square to the next, which sweeps a quarter of a
cylinder of radius 3 and height 6. The bar closes on itself with no ends, a flat ring: flat top
and bottom at z = ±3, an outer wall rounded at the four corners, an inner wall sharp at them."""
from corner import ROUND, polyline
from pipes import pipe, start_frame
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.gp import gp_Dir, gp_Pnt


def build():
    path = polyline([(20, 0, 0), (40, 0, 0), (40, 30, 0), (0, 30, 0), (0, 0, 0)], closed=True)
    section = BRepBuilderAPI_MakePolygon()
    for y, z in [(3, 3), (-3, 3), (-3, -3), (3, -3)]:
        section.Add(gp_Pnt(20, y, z))
    section.Close()
    return pipe(path, section.Wire(), binormal=gp_Dir(0, 0, 1), corners=ROUND)
