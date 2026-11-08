"""A pointed drill swung partway into a curved block. A curved block with a notch cut down into it
by a drill swung partway into it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The drill is a solid of revolution about the vertical line through (10, 0): a cylinder of radius 1
from z = −0.8 + 1/tan 59° up to z = 3, ended below in a cone of 118° (each side 59° from the axis)
whose point is (10, 0, −0.8). The point is in the block; the drill stands out through its top.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the drill passes
through.

The rim where the cone meets the cylinder is a sharp circle. The turn carries the cone's side and
the cylinder's into cones and cylinders about the z axis, which meet along the circles the rim's
nearest and farthest points sweep."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax1, gp_Pnt

from sweep import cut, plane, ring_sector, turn, turned

BLOCK = (8, 12, -1, 1, 0, 60)  # radii, heights and polar angles
AXIS = 10  # the drill's axis is the vertical line through (10, 0)
POINT, TOP = -0.8, 3  # heights of its point and its flat top
RADIUS = 1
HALF_ANGLE = 59  # of the point, from the axis
START, STOP = -140, 30  # degrees about the z axis


def build():
    shoulder = POINT + RADIUS / math.tan(math.radians(HALF_ANGLE))
    # the drill's half-section beside its axis, turned about the axis: a cone and a cylinder
    profile = [gp_Pnt(AXIS, 0, POINT), gp_Pnt(AXIS + RADIUS, 0, shoulder),
               gp_Pnt(AXIS + RADIUS, 0, TOP), gp_Pnt(AXIS, 0, TOP)]
    half = BRepBuilderAPI_MakeFace(BRepBuilderAPI_MakePolygon(*profile, True).Wire()).Face()
    drill = BRepPrimAPI_MakeRevol(half, gp_Ax1(gp_Pnt(AXIS, 0, 0), gp.DZ_s())).Shape()
    tool = turn(drill, gp.OZ_s(), START)
    # The turn's axis and the drill's are parallel: its cone and its cylinder meet the turn
    # square-on where their normals lie in the plane through both axes, which cuts them along
    # their contact; the flat top turns in its own plane.
    through_axes = turn(plane((0, 0, 0), (0, 1, 0)), gp.OZ_s(), START)
    swept = turned(tool, gp.OZ_s(), STOP - START, [through_axes])
    return cut(ring_sector(*BLOCK), swept)
