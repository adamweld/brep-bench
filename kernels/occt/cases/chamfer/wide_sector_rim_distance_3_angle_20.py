"""A wide spherical sector's rim chamfered by a distance of 3 and an angle of 20°. A wide spherical sector: the part of a ball of radius 5 centered on the origin that lies inside the cone
with its apex at the origin, about the z axis, opening upward with half angle α, cos α = 5/13 (about
67.38°). The cone's side meets the ball in the circle of radius 60/13 at z = 25/13, the sector's rim.
Chamfer the rim by a distance and an angle: 3 down the cone from the rim, the chamfer making 20°
with the cone's side.

In the plane through the z axis the cone's side is the straight line from the origin to the rim. The
chamfer starts on it at P, 3 from the rim, (1.846, 0.7692) in (ρ, z), and is the straight line
from P that makes 20° with the cone's line continued toward the rim, turned toward the z axis, into
the sector. It runs 3.074 to meet the ball at Q ≈ (4.108, 2.85). The chamfer is the cone
that line sweeps about the z axis, and the solid is the sector cut by it: the cone up to P, the
chamfer from P to Q, and the ball above Q.

The angle is measured from the cone's side, which is straight in that plane, so it is the same
angle everywhere along the chamfer's start."""
from math import cos, radians, sin

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeSphere

from chamfer import cut_beyond_line

DOWN_CONE = 3
ANGLE = radians(20)
SIDE = (12 / 13, 5 / 13)  # the cone's side in (ρ, z), from the apex toward the rim


def build():
    ball = BRepPrimAPI_MakeSphere(5).Shape()
    # the cone with its apex at the origin, radius 12 at z = 5
    cone = BRepPrimAPI_MakeCone(0, 12, 5).Shape()
    sector = BRepAlgoAPI_Common(ball, cone).Shape()
    # BRepFilletAPI_MakeChamfer's distance-angle chamfer (AddDA) builds this rim, but its bevel is
    # a B-spline fitted to the cone, a few 1e-7 off. The solid is built here as the case states it:
    # the sector cut by the chamfer's cone, whose line starts DOWN_CONE from the rim on the cone's
    # side and leaves that side at ANGLE, turned toward the axis.
    start = ((5 - DOWN_CONE) * SIDE[0], (5 - DOWN_CONE) * SIDE[1])
    along = (SIDE[0] * cos(ANGLE) - SIDE[1] * sin(ANGLE),
             SIDE[0] * sin(ANGLE) + SIDE[1] * cos(ANGLE))
    return cut_beyond_line(sector, start, along, "the sector by its chamfer's cone")
