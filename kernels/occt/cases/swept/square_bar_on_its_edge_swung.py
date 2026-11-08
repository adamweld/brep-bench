"""A square bar standing on its edge, swung a quarter turn. The solid a square bar fills as it is
swung a quarter turn about an axis it faces with an edge.

The bar is a prism standing upright from z = 0 to z = 3. Its section is the square with corners (4,
0), (6, 2), (8, 0) and (6, −2): stood on a corner, its diagonal along the x axis, so that one of
its upright edges, the line through (4, 0), faces the z axis squarely.

It turns rigidly about the z axis, right-handed (from +x toward +y), by 90°. Make the solid of
everything the bar passes through.

The edges through (4, 0) and (8, 0) are in contact all along them, the fan of normals at each
holding the direction to the axis: they sweep cylinders of radii 4 and 8 about it. The bar's faces
touch nowhere. What it sweeps is a quarter of the ring between those cylinders, with the bar where
it starts and where it stops at its ends."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp, gp_Pnt, gp_Vec

from sweep import flat_contacts_turned, turned, united

SECTION = [(4, 0), (6, 2), (8, 0), (6, -2)]
HEIGHT = 3
TURN = 90  # degrees about the z axis


def build():
    square = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakePolygon(
        *(gp_Pnt(x, y, 0) for x, y in SECTION), True).Wire()).Face()
    bar = BRepPrimAPI_MakePrism(square, gp_Vec(0, 0, HEIGHT)).Shape()
    # Each upright face meets the turn square-on along the plane through the axis and its
    # normal, none of which reaches it: they lead or trail all over.
    return united(turned(bar, gp.OZ_s(), TURN, flat_contacts_turned(bar, gp.OZ_s())))
