"""A ball in a conical seat, the rim between them chamfered by a distance of 3 and an angle of 15°. A ball of radius 5 about the origin standing in a conical seat. The seat is a plate, the cylinder of
radius 8 about the z axis from z = −6 to z = 5, with a cone hollowed out of its top: everything above the
cone z = 1 + r/2 (its apex at (0, 0, 1), opening upward, its side at about 63.4° to the axis), so the
seat slopes down and inward from the plate's top rim at radius 8, z = 5. The ball is joined to it; the
seat's cone meets the ball in the circle of radius 4 at z = 3, a concave edge, the ball standing out of
the seat above it.

Chamfer that edge by a distance and an angle: 3 up the seat's cone from the edge, the chamfer making
15° with the cone, rising from it back toward the ball and filling the groove. In the plane through the z
axis the cone's side is the straight line from the edge (4, 3) up and out to (8, 5). The chamfer starts on
it at P, 3 from the edge, (6.683, 4.342) in (ρ, z), and is the straight line from P that makes 15°
with the cone's line back toward the edge, turned upward, until it first meets the ball, 3.355 along, at
Q ≈ (3.397, 3.669). The chamfer is the cone that line sweeps about the z axis, and the solid is the
seat and the ball with the ring between them and that cone filled in.

The angle is measured from the seat's cone, which is straight in that plane, so it is the same angle
everywhere along the chamfer's start."""
from math import atan2, cos, radians, sin, sqrt

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from chamfer import filled_by_revolved

UP_CONE = 3
ANGLE = radians(15)


def build():
    up = gp_Dir(0, 0, 1)
    plate = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -6), up), 8, 11).Shape()
    hollow = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 1), up), 0, 8, 4).Shape()
    seat = BRepAlgoAPI_Cut(plate, hollow).Shape()
    joined = BRepAlgoAPI_Fuse(seat, BRepPrimAPI_MakeSphere(5).Shape())
    if not joined.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the seat and the ball")
    # BRepFilletAPI_MakeChamfer's distance-angle chamfer (AddDA) fits a B-spline to the bevel. The
    # solid is built here as the case states it: the groove filled up to the chamfer's cone, whose
    # line starts UP_CONE up the seat from the edge and leaves it at ANGLE back toward the ball, to
    # its first crossing of the ball. The fill is the triangle from a point of the seat's cone line
    # inside the ball, through the start, to the end, turned about the axis; its part inside the
    # ball is the ball's already.
    up_cone = (2 / sqrt(5), 1 / sqrt(5))
    start = (4 + UP_CONE * up_cone[0], 3 + UP_CONE * up_cone[1])
    heading = atan2(-up_cone[1], -up_cone[0]) - ANGLE
    along = (cos(heading), sin(heading))
    toward = start[0] * along[0] + start[1] * along[1]
    run = -toward - sqrt(toward ** 2 - (start[0] ** 2 + start[1] ** 2 - 25))
    end = (start[0] + run * along[0], start[1] + run * along[1])
    inside = (4 - up_cone[0], 3 - up_cone[1])
    return filled_by_revolved(joined.Shape(), [inside, start, end], "the groove up to the chamfer's cone")
