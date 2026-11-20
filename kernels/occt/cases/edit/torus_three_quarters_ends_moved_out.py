"""Three quarters of a torus, both its ends moved out. Three quarters of the solid torus about the z axis whose tube, of radius 3, runs round the circle
of radius 10 in the plane z = 0: the part whose angle about the z axis, φ, measured from +x toward
+y, lies between 0° and 270°. Its faces are the torus and two flat disks of radius 3, its ends, in
the half-planes y = 0, x ≥ 0 (φ = 0°) and x = 0, y ≤ 0 (φ = 270°).

Then edit the solid directly: move the end at φ = 0° by 2 outward along its normal, −y, to the
plane y = −2, and the end at φ = 270° by 2 outward along its normal, +x, to the plane x = 2. The
torus runs on round into the quarter that was missing, as far as them: each end is now the section
of the tube by that plane, no longer a disk, y = −2 with x ≥ 2 and x = 2 with y ≤ −2. The two ends
do not meet.

So the result is the torus less its part with x > 2 and y < −2."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp

from bench import face_through
from edit import moved

MAJOR, MINOR, DISTANCE = 10, 3, 2


def build():
    torus = BRepPrimAPI_MakeTorus(gp.XOY_s(), MAJOR, MINOR, 3 * math.pi / 2).Shape()
    first = face_through(torus, (10, 0, 1))  # the disk at φ = 0°, its outward normal −y
    last = face_through(torus, (0, -10, 1))  # the disk at φ = 270°, its outward normal +x
    return moved(torus, [(first, DISTANCE), (last, DISTANCE)])
