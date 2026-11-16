"""A ball on a plate, the concave edge between them chamfered by a distance of 2 and an angle of
40°. A plate, the cylinder of radius 8 on the z axis from z = −6 to z = 0, with a ball of radius
5 centered on the origin standing in it: the ball's upper half rises from the plate's top face.
The ball meets that face in the circle of radius 5 at z = 0, a concave edge. Chamfer it by a
distance and an angle: 2 out from the edge on the plate's top, the chamfer making 40° with the
plate. A concave chamfer adds material: it fills the groove between the plate and the ball.

In the plane through the z axis the plate's top is the straight line z = 0 from the edge out to the
plate's rim. The chamfer starts on it at (7.0, 0), 2 out from the edge, and is the straight line
from there that rises at 40° back toward the ball until it meets it, 3.182 along, at Q ≈ (4.563,
2.045). The chamfer is the cone that line sweeps about the z axis, and the solid is the plate and
the ball with the ring between them and that cone filled in.

The angle is measured from the plate's top, which is flat."""
from math import cos, radians, sin, sqrt

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from chamfer import filled_by_revolved

ON_PLATE = 2
ANGLE = radians(40)


def build():
    plate = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -6), gp_Dir(0, 0, 1)), 8, 6).Shape()
    ball = BRepPrimAPI_MakeSphere(5).Shape()
    joined = BRepAlgoAPI_Fuse(plate, ball)
    if not joined.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the plate and the ball")
    # BRepFilletAPI_MakeChamfer's distance-angle chamfer (AddDA) fits a B-spline to the bevel. The
    # solid is built here as the case states it: the groove filled up to the chamfer's cone, whose
    # line starts ON_PLATE out from the edge on the plate and rises at ANGLE back to the ball. The
    # fill is the triangle edge, start, end turned about the axis; its part inside the ball is
    # the ball's already.
    start = 5 + ON_PLATE
    along = start * cos(ANGLE) - sqrt((start * cos(ANGLE)) ** 2 - (start * start - 25))
    end = (start - along * cos(ANGLE), along * sin(ANGLE))
    return filled_by_revolved(joined.Shape(), [(5, 0), (start, 0), end],
                              "the groove up to the chamfer's cone")
