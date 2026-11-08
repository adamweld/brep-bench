"""A ball swung about an axis that passes through it. The solid a ball fills as it is swung partway
about an upright line that passes through it.

The ball has radius 2 and its center is (1, 0, 0). It turns rigidly about the z axis, right-handed
(from +x toward +y), by 120°, its center running along an arc of radius 1. Make the solid of
everything the ball passes through.

The ball reaches across the turn's axis, so the tube about its center's arc is a piece of a spindle
torus: it passes through itself on the axis, and its inner part is buried inside the balls at the
two ends."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Pnt

from sweep import plane, turned, united

CENTRE = (1, 0, 0)
RADIUS = 2
TURN = 120  # degrees about the z axis


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(*CENTRE), RADIUS).Shape()
    # A point p of the ball moves with velocity ẑ × p, square to its normal (p − c)/2 where
    # (ẑ × c)·p = 0: the great circle in the plane y = 0 through the axis and the centre. That
    # circle crosses the axis, and a curve turned about an axis it crosses sweeps a surface
    # that passes through itself, which MakeRevol declines: the plane x = 0 through the axis
    # cuts the ball again where the circle crosses, so that each piece of it lies to one side.
    cuts = [plane((0, 0, 0), (0, 1, 0)), plane((0, 0, 0), (1, 0, 0))]
    return united(turned(ball, gp.OZ_s(), TURN, cuts))
