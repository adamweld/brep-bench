"""A quarter of a ball, both its flats moved out past its axis. A quarter of the ball of radius 10
centered on the origin, from pole to pole: the part whose angle about the z axis, φ, measured
from +x toward +y, lies between 0° and 90°, so the part with x ≥ 0 and y ≥ 0. Its poles are on
the z axis, at (0, 0, ±10). Its faces are the sphere and two flat half disks, in the half-plane y
= 0, x ≥ 0 and in the half-plane x = 0, y ≥ 0, which meet along the z axis in a convex edge from
pole to pole.

Then edit the solid directly: move each flat 4 outward along its normal, the first to the plane y
= −4 and the second to the plane x = −4. The faces beside them follow: the sphere runs on round
past the old edge to meet both moved flats, and the two flats are extended until they meet each
other, along the line x = y = −4 from z = −√68 to z = √68. The poles, where the old edge met the
sphere, are left inside the spherical face.

So the result is the ball less everything at x < −4 or y < −4. Each flat is the disk of radius
√84 in which its plane cuts the ball, less the segment beyond the other flat's plane."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from bench import face_through
from edit import moved

RADIUS, DISTANCE = 10, 4


def build():
    lune = BRepPrimAPI_MakeSphere(RADIUS, math.pi / 2).Shape()  # 0° ≤ φ ≤ 90°
    flats = [face_through(lune, (3, 0, 1)), face_through(lune, (0, 3, 1))]
    return moved(lune, [(flat, DISTANCE) for flat in flats])
