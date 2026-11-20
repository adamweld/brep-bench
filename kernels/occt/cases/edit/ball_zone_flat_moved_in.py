"""A ball's zone from its south pole, its flat moved in. The part of the ball of radius 10 centered on the origin that lies below the plane z = 3, from its
south pole, (0, 0, −10), up to that latitude. Its faces are the spherical zone and one flat disk of
radius √91 in the plane z = 3, facing up.

Then edit the solid directly: move the flat 4 inward along its normal, down to the plane z = −1. The
sphere is trimmed back to meet it, so the flat becomes a disk of radius √99.

So the result is the ball less everything above z = −1: its zone from the south pole to the latitude
z = −1, and a flat disk of radius √99 closing it."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import face_through
from edit import moved

def build():
    zone = BRepPrimAPI_MakeSphere(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), 10,
                                  -math.pi / 2, math.asin(0.3)).Shape()  # up to z = 3
    flat = face_through(zone, (1, 0, 3))
    return moved(zone, [(flat, -4)])
