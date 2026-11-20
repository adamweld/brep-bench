"""The upper half of a horn torus, its flat moved up through the pinch. A horn torus: the solid swept by the disk of radius 5 centered on (5, 0, 0) in the plane y = 0,
turned a full turn about the z axis. Its tube touches the axis at the origin, where its surface
pinches to a point. Take its upper half, z ≥ 0. Its faces are the torus and a flat disk of radius
10 in the plane z = 0, whose center is the pinch.

Then edit the solid directly: move the flat 3 inward, up against its outward normal, to the plane
z = 3. The torus is trimmed back to z ≥ 3, so the flat becomes the annulus between radii 1 and 9
in the plane z = 3, and the pinch at the origin, now cut away, is no longer part of the solid.

So the result is the horn torus less its part below z = 3: a ring with a hole through its middle."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus

from bench import face_through
from edit import moved

RADIUS, DISTANCE = 5, 3


def build():
    half = BRepPrimAPI_MakeTorus(RADIUS, RADIUS, 0, math.pi).Shape()  # the tube's upper half
    flat = face_through(half, (3, 1, 0))
    return moved(half, [(flat, -DISTANCE)])  # its outward normal is −z
