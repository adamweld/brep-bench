"""A floor cut by a leaning bar's flat end, turned. A curved block whose floor is cut by the flat
end of a leaning bar turned past it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The bar is drawn in the plane through (10, 0, 0) spanned by u = (cos 20°, 0, −sin 20°) and v = (0,
1, 0): a point (a, b) of that plane is (10, 0, 0) + a u + b v. Its end is the rectangle 3 by 4
there turned 5° in that plane, {(a, b): 0 ≤ a cos 5° + b sin 5° ≤ 3, |b cos 5° − a sin 5°| ≤ 2},
whose side through (10, 0, 0) is the bar's inner side. The bar is that rectangle carried 3 along n
= (sin 20°, 0, cos 20°), square to the plane: it leans 20° out from upright, its flat end facing
down and toward the z axis.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +140°: it starts
clear of the block, passes through it, and ends clear of it. Cut from the block everything the bar
passes through.

The end touches along the line where it meets the plane through the z axis and its normal, which
the turn carries round into the cut's floor, a cone; the inner side touches along a line crossing
its rulings; the rim between them carries the contact from the one to the other. The section is
turned 5° so that no side faces the axis squarely."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp, gp_Pnt, gp_Vec

from sweep import cut, flat_contacts_turned, ring_sector, turn, turned

BLOCK = (8, 12, -1, 1, 0, 60)  # radii, heights and polar angles
ORIGIN = (10, 0, 0)
LEAN = math.radians(20)
TWIST = math.radians(5)  # the end's rectangle, turned in its plane
LENGTH, HALF_WIDTH = 3, 2  # of the rectangle, along a and across it
DEPTH = 3  # the bar's end carried along n
START, STOP = -140, 140  # degrees about the z axis


def build():
    u = (math.cos(LEAN), 0, -math.sin(LEAN))
    v = (0, 1, 0)
    n = (math.sin(LEAN), 0, math.cos(LEAN))
    c, s = math.cos(TWIST), math.sin(TWIST)
    # the rectangle's corners, (a, b) = p(cos 5°, sin 5°) + q(−sin 5°, cos 5°) for p from 0 to 3
    # and q from −2 to 2, placed at (10, 0, 0) + a u + b v
    corners = []
    for p, q in ((0, -HALF_WIDTH), (LENGTH, -HALF_WIDTH), (LENGTH, HALF_WIDTH), (0, HALF_WIDTH)):
        a, b = p * c - q * s, p * s + q * c
        corners.append(gp_Pnt(*(o + a * x + b * y for o, x, y in zip(ORIGIN, u, v))))
    end = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakePolygon(*corners, True).Wire()).Face()
    bar = turn(BRepPrimAPI_MakePrism(end, gp_Vec(*(DEPTH * x for x in n))).Shape(), gp.OZ_s(),
               START)
    # Each flat face meets the turn square-on along the plane through the axis and its normal,
    # which cuts it along its contact where it reaches it.
    swept = turned(bar, gp.OZ_s(), STOP - START, flat_contacts_turned(bar, gp.OZ_s()))
    return cut(ring_sector(*BLOCK), swept)
