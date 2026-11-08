"""A rod spun partway about a line through it. The solid a rod fills as it is spun partway about an
upright line that passes through it.

The rod is a solid cylinder of radius 0.5 whose axis is the x axis, from x = −3 to x = 5, its ends
flat. The z axis crosses it square to its axis.

It turns rigidly about the z axis, right-handed (from +x toward +y), by 120°. Make the solid of
everything the rod passes through.

The turn's axis crosses the rod's. Every circle of the rod about its own axis touches at its top
and bottom points, z = ±0.5, except the one at x = 0, which the turn moves along the rod's own axis
and which touches all round: there the two roots cross. The rod's two arms sweep two fans, of radii
5 and 3, that meet about the axis."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from sweep import plane, turned, united

RADIUS, NEAR, FAR = 0.5, -3, 5  # the rod, along the x axis
TURN = 120  # degrees about the z axis


def build():
    rod = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(NEAR, 0, 0), gp.DX_s()), RADIUS,
                                   FAR - NEAR).Shape()
    # The rod's axis meets the turn's, at the origin. At the point (x, ½ cos t, ½ sin t) of its
    # side, its normal (0, cos t, sin t) is square to the velocity ẑ × p = (−½ cos t, x, 0) where
    # x cos t = 0: along its lines in the plane y = 0 through both axes (top and bottom), and
    # all round its circle at x = 0, where the turn moves it along the rod's own axis. Its flat
    # ends meet the turn square-on along the plane y = 0 too.
    cuts = [plane((0, 0, 0), (0, 1, 0)), plane((0, 0, 0), (1, 0, 0))]
    return united(turned(rod, gp.OZ_s(), TURN, cuts))
