"""Three quarters of a ball, both its flats moved out. Three quarters of the ball of radius 5 about the origin: the part whose angle about the z axis, φ,
measured from +x toward +y, lies between 0° and 270°. Its faces are the sphere and two flat
half-disks of radius 5 through the axis, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the
half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the axis, the diameter from (0, 0, −5) to (0,
0, 5), in a concave edge.

Then edit the solid directly: move the flat at φ = 0° by 2 outward along its normal, −y, to the
plane y = −2, and the flat at φ = 270° by 2 outward along its normal, +x, to the plane x = 2. The
faces beside them follow: the sphere runs on round into the quarter that was missing, as far as
the moved flats. The flat in y = −2 is now the part of the section of the ball there with x ≥ 2,
and the flat in x = 2 the part of its section with y ≤ −2; they meet along the line x = 2, y = −2,
in a concave edge from the sphere to the sphere.

So the result is the ball less its part with x > 2 and y < −2."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp

from bench import face_through
from edit import moved

RADIUS, DISTANCE = 5, 2


def build():
    ball = BRepPrimAPI_MakeSphere(gp.XOY_s(), RADIUS, 3 * math.pi / 2).Shape()
    first = face_through(ball, (3, 0, 1))   # the half-disk at φ = 0°, its outward normal −y
    last = face_through(ball, (0, -3, 1))   # the half-disk at φ = 270°, its outward normal +x
    return moved(ball, [(first, DISTANCE), (last, DISTANCE)])
