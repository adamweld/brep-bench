"""Three quarters of a torus, one end moved in. Three quarters of the solid torus about the z axis whose tube, of radius 3, runs round the circle
of radius 10 in the plane z = 0: the part whose angle about the z axis, φ, measured from +x toward
+y, lies between 0° and 270°. Its faces are the torus and two flat disks of radius 3, its ends, in
the half-planes y = 0, x ≥ 0 (φ = 0°) and x = 0, y ≤ 0 (φ = 270°).

Then edit the solid directly: move the end at φ = 0° by 2 inward along its normal, +y, to the
plane y = 2. The torus is trimmed back to meet it, so that end is now the section of the tube by y
= 2, with x ≥ 0, no longer a disk. The other end, in x = 0, is untouched, and the two ends are far
apart.

So the result is the torus less its part with x > 0 and y < 2."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp

from bench import face_through
from edit import moved

MAJOR, MINOR, DISTANCE = 10, 3, 2


def build():
    torus = BRepPrimAPI_MakeTorus(gp.XOY_s(), MAJOR, MINOR, 3 * math.pi / 2).Shape()
    end = face_through(torus, (10, 0, 1))  # the disk at φ = 0°, its outward normal −y
    return moved(torus, [(end, -DISTANCE)])
