"""Three quarters of a ball, one flat moved in. Three quarters of the ball of radius 5 about the origin: the part whose angle about the z axis, φ,
measured from +x toward +y, lies between 0° and 270°. Its faces are the sphere and two flat
half-disks of radius 5 through the axis, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the
half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the axis, the diameter from (0, 0, −5) to (0,
0, 5), in a concave edge.

Then edit the solid directly: move the flat at φ = 0° by 2 inward along its normal, +y, to the
plane y = 2. The faces beside it follow it: the sphere is trimmed back to y ≥ 2 where x > 0, and
the other flat, in x = 0, runs on past the axis, from y ≤ 0 to y ≤ 2, to meet it. The two flats
meet along the line x = 0, y = 2, in a concave edge from the sphere to the sphere. The moved flat
is the half-disk of radius √21 in the plane y = 2, x ≥ 0.

So the result is the ball less its part with x > 0 and y < 2."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp

from bench import face_through
from edit import moved

RADIUS, DISTANCE = 5, 2


def build():
    ball = BRepPrimAPI_MakeSphere(gp.XOY_s(), RADIUS, 3 * math.pi / 2).Shape()
    flat = face_through(ball, (3, 0, 1))  # the half-disk at φ = 0°, its outward normal −y
    return moved(ball, [(flat, -DISTANCE)])
