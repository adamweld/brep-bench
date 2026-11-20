"""A quarter of a torus, one end moved out. A quarter of the solid torus about the z axis whose tube, of radius 3, runs round the circle of
radius 10 in the plane z = 0: the part whose angle about the z axis, φ, measured from +x toward
+y, lies between 0° and 90°. Its faces are the torus and two flat disks of radius 3, its ends, in
the half-planes y = 0, x ≥ 0 (φ = 0°) and x = 0, y ≥ 0 (φ = 90°).

Then edit the solid directly: move the end at φ = 0° by 2 outward along its normal, −y, to the
plane y = −2. The torus runs on as the same surface into the part before φ = 0°, as far as that
plane, which cuts it in a flat face that is no longer a disk: the section of the tube by y = −2,
with x ≥ 0. The other end is untouched, a quarter of a turn away.

So the result is the quarter and the part of the torus with x ≥ 0 and −2 ≤ y ≤ 0."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp

from bench import face_through
from edit import moved

MAJOR, MINOR, DISTANCE = 10, 3, 2


def build():
    quarter = BRepPrimAPI_MakeTorus(gp.XOY_s(), MAJOR, MINOR, math.pi / 2).Shape()
    end = face_through(quarter, (10, 0, 1))  # the disk at φ = 0°, its outward normal −y
    return moved(quarter, [(end, DISTANCE)])
