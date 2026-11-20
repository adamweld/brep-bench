"""A pointed cone's base moved out. The cone whose base, of radius 6, lies in the plane z = 0 centered on the origin and whose apex is
(0, 0, 9). Its faces are the cone and the flat base disk.

Then edit the solid directly: move the base 3 outward along its normal, down, to the plane z = −3.
The cone runs on below z = 0 to meet it, to a base of radius 8 about the same axis; the apex does
not move.

So the result is the cone with apex (0, 0, 9) and base of radius 8 in the plane z = −3, 12 high."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp

from bench import face_through
from edit import moved

RADIUS, HEIGHT, DISTANCE = 6, 9, 3


def build():
    cone = BRepPrimAPI_MakeCone(gp.XOY_s(), RADIUS, 0, HEIGHT).Shape()
    base = face_through(cone, (1, 1, 0))
    return moved(cone, [(base, DISTANCE)])  # its outward normal is −z
