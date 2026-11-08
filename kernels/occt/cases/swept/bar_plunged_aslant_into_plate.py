"""A bar plunged aslant into a plate, stopping inside. A plate, the box from (0, −5, 0) to (20, 5,
4), with a pocket cut into its underside by a bar pushed up aslant into it, stopping halfway
through.

The bar is a prism of square section stood on a corner: everything with 6 ≤ x ≤ 14 and |y| + |z +
6| ≤ 1. Its section's corners are (y, z) = (0, −7), (1, −6), (0, −5) and (−1, −6).

It slides without turning by 8 along the direction (0.3, 0, √(1 − 0.3²)), leaning 0.3 toward +x
from straight up, so that it ends with its top edge at z = −5 + 8√0.91 ≈ 2.63, inside the plate.
Cut from the plate everything the bar passes through.

The bar is convex, so what it passes through is the convex hull of the bar where it starts and the
bar where it stops; the pocket is the part of that within the plate."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from sweep import cut, slid

PLATE = ((0, -5, 0), (20, 5, 4))
BAR_START, BAR_LENGTH = 6, 8  # along x
SECTION = [(0, -7), (1, -6), (0, -5), (-1, -6)]  # the square's corners, in (y, z)
LEAN = 0.3  # the push's x component, of a unit direction
PUSH = 8


def build():
    corners = [gp_Pnt(BAR_START, y, z) for y, z in SECTION]
    section = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakePolygon(*corners, True).Wire()).Face()
    bar = BRepPrimAPI_MakePrism(section, gp_Vec(BAR_LENGTH, 0, 0)).Shape()
    push = (PUSH * LEAN, 0, PUSH * math.sqrt(1 - LEAN ** 2))
    # Every face of the bar is flat and none is parallel to the push, so each leads all over or
    # trails all over: none needs splitting.
    swept = slid(bar, push)
    return cut(BRepPrimAPI_MakeBox(gp_Pnt(*PLATE[0]), gp_Pnt(*PLATE[1])).Shape(), swept)
