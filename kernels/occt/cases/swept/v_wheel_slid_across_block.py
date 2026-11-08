"""A V groove cut by a sharp V wheel slid across a block. A block, the box from (0, −5, 0) to (20,
5, 10), with a V groove cut across its top by a wheel with a sharp rim.

The wheel's axis is the line through (−8, 0, 14) parallel to the y axis. The wheel is the solid of
revolution, about that axis, of the triangle in the plane x = −8 with corners, in (y, z), at (0,
8), (−3 tan 30°, 11) and (3 tan 30°, 11): its rim is a sharp circle of radius 6 whose lowest point,
(−8, 0, 8), is 2 below the block's top, and its flanks lean 30° either side of its mid-plane (a 60°
V), back to its bore of radius 3.

It slides without turning along the x axis by 36, from clear before the block to clear past it. Cut
from the block everything the wheel passes through.

Each flank touches along its line square to the slide, and the two lines meet at the rim's lowest
point, where every normal between the flanks is the wheel's. The groove is the V of those lines
carried along the block: two planes meeting in a sharp edge 2 deep, the rim's lowest point's
path."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax1, gp_Pnt

from sweep import cut, plane, slid

BLOCK = ((0, -5, 0), (20, 5, 10))
CENTRE = (-8, 0, 14)  # on the wheel's axis, parallel to y
SECTION = [(0, 8), (-3 * math.tan(math.radians(30)), 11), (3 * math.tan(math.radians(30)), 11)]
SLIDE = (36, 0, 0)


def build():
    x = CENTRE[0]
    corners = [gp_Pnt(x, y, z) for y, z in SECTION]
    triangle = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakePolygon(*corners, True).Wire()).Face()
    wheel = BRepPrimAPI_MakeRevol(triangle, gp_Ax1(gp_Pnt(*CENTRE), gp.DY_s())).Shape()
    # The wheel's flanks are cones and its bore a cylinder, all about an axis square to the
    # slide (x): each meets the slide square-on along its lines through the axis's plane x = −8,
    # where its normal is square to x, and leads on one side of it and trails on the other.
    swept = slid(wheel, SLIDE, [plane(CENTRE, (1, 0, 0))])
    return cut(BRepPrimAPI_MakeBox(gp_Pnt(*BLOCK[0]), gp_Pnt(*BLOCK[1])).Shape(), swept)
