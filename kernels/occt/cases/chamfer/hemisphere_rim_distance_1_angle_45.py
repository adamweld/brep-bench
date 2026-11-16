"""A hemisphere's rim chamfered by a distance of 1 and an angle of 45°. The upper half of a ball of
radius 5 centered on the origin, z ≥ 0. Chamfer the circle where its flat face z = 0 meets the
ball by a distance and an angle: 1 in from the rim on the flat face, the chamfer making 45° with
the flat face.

In the plane through the z axis the flat face is the straight line z = 0 out to the rim. The chamfer
starts on it at (4.0, 0), 1 in from the rim, and is the straight line from there that rises at 45°
toward the rim's side until it meets the ball, 1.295 along, at Q ≈ (4.915, 0.9155). The chamfer is
the cone that line sweeps about the z axis, and the solid is the hemisphere cut by it: the flat face
shrinks to the disk of radius 4.0, and the ball keeps everything above Q.

The angle is measured from the flat face. On a ball the chamfer's two legs are not equal even at
45°: the line reaches the ball 0.9155 above the flat, not 1."""
from math import cos, radians, sin

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from chamfer import cut_beyond_line

ON_FLAT = 1
ANGLE = radians(45)


def build():
    ball = BRepPrimAPI_MakeSphere(5).Shape()
    upper = BRepPrimAPI_MakeBox(gp_Pnt(-20, -20, 0), 40, 40, 20).Shape()
    hemisphere = BRepAlgoAPI_Common(ball, upper).Shape()
    # BRepFilletAPI_MakeChamfer's distance-angle chamfer (AddDA) builds this rim, but its bevel is
    # a B-spline fitted to the cone. The solid is built here as the case states it: the hemisphere
    # cut by the chamfer's cone, whose line starts ON_FLAT in from the rim on the flat face and
    # rises from it at ANGLE.
    return cut_beyond_line(hemisphere, (5 - ON_FLAT, 0), (cos(ANGLE), sin(ANGLE)),
                           "the hemisphere by its chamfer's cone")
