"""A ball standing in a plate, its beveled root deleted: the plate and the ball meet again. A plate, the disk of radius 8 about the z axis from z = −2 to z = 0, and on it the ball of radius 3 about
(0, 0, 1), joined to it: the ball stands up out of the plate's top through the circle of radius √8 at z = 0.
The concave edge where they meet is beveled: the frustum from radius 4 at z = 0 to radius 3 at z = 1 is joined
too, so a cone runs from the plate's top at radius 4 up and in to the ball's equator, the circle of radius 3
at z = 1, filling the groove.

Then edit it directly: delete the bevel. Deleting a face takes it out of the solid and closes the gap it
leaves with the faces round it, each run on along its own surface until they meet. The top runs on inward and
the ball down from its equator until they meet in the circle of radius √8 at z = 0.

The result is the plate and the ball joined, their edge sharp."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from edit import cylinder, deleted, frustum, united
from bench import face_through


def build():
    plate = cylinder((0, 0, -2), 8, 2)
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 1), 3).Shape()
    beveled = united(united(plate, ball), frustum((0, 0, 0), 4, 3, 1))
    # The bevel, halfway along its slant.
    return deleted(beveled, [face_through(beveled, (0.0, -3.5, 0.5))])
