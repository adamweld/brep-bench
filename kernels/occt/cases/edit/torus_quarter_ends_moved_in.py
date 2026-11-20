"""A quarter of a torus, both its ends moved in. A quarter of the solid torus about the z axis whose tube, of radius 3, runs round the circle of
radius 10 in the plane z = 0: the part whose angle about the z axis, φ, measured from +x toward
+y, lies between 0° and 90°. Its faces are the torus and two flat disks of radius 3, its ends, in
the half-planes y = 0, x ≥ 0 (φ = 0°) and x = 0, y ≥ 0 (φ = 90°).

Then edit the solid directly: move each end 2 inward along its normal, the one at φ = 0° up to the
plane y = 2 and the one at φ = 90° across to the plane x = 2. The torus is trimmed back to meet
them, and each end is now a flat face that is no longer a disk: the section of the tube by y = 2,
with x ≥ 2, and by x = 2, with y ≥ 2. The two ends are far apart.

So the result is the part of the torus with x ≥ 2 and y ≥ 2."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp

from bench import face_through
from edit import moved

MAJOR, MINOR, DISTANCE = 10, 3, 2


def build():
    quarter = BRepPrimAPI_MakeTorus(gp.XOY_s(), MAJOR, MINOR, math.pi / 2).Shape()
    first = face_through(quarter, (10, 0, 1))  # the disk at φ = 0°, its outward normal −y
    last = face_through(quarter, (0, 10, 1))   # the disk at φ = 90°, its outward normal −x
    return moved(quarter, [(first, -DISTANCE), (last, -DISTANCE)])
