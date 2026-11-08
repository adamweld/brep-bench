"""A disc turned about an upright line through its rim. The solid a thick disc fills as it is
turned partway about an upright line that passes through its rim.

The disc is a solid cylinder of radius 2 whose axis is the line through (2, 0, 0) parallel to the y
axis, from y = 0 to y = 0.5, its faces flat. Its rim in the plane y = 0, a sharp circle of radius
2, passes through the origin, on the z axis.

It turns rigidly about the z axis, right-handed (from +x toward +y), by 90°. Make the solid of
everything the disc passes through.

Each rim is a sharp circle leaning to the turn's velocity; the one through the axis sweeps a
surface of revolution that closes on the axis, and the disc's flat face there turns about a line in
its own plane."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from sweep import plane, turned, unified, united

CENTRE = (2, 0, 0)  # the middle of the face y = 0, on the disc's axis
RADIUS, THICK = 2, 0.5
TURN = 90  # degrees about the z axis


def build():
    coin = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*CENTRE), gp.DY_s()), RADIUS, THICK).Shape()
    # At the point (2 + 2 cos t, y, 2 sin t) of the disc's side, its normal (cos t, 0, sin t) is
    # square to the velocity ẑ × p = (−y, 2 + 2 cos t, 0) where y cos t = 0: on its rim in the
    # face y = 0, and along its two lines at x = 2 (top and bottom), where the plane x = 2 cuts
    # it. Its flat faces meet the turn square-on along the plane x = 0 through the axis and
    # their normal, which reaches them only at a point of their rims.
    return unified(united(turned(coin, gp.OZ_s(), TURN, [plane(CENTRE, (1, 0, 0))])))
