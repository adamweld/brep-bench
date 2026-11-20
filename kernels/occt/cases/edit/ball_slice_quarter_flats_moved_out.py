"""A quarter of a slice of a ball, both its side flats moved out. The ball of radius 10 centered on the
origin, sliced by the planes z = 0 and z = 6, and a quarter of the slice: the part with x ≥ 0 and y
≥ 0, whose angle φ about the z axis lies between 0° and 90°. Its faces are the sphere; a flat
bottom, a quarter disk of radius 10 in the plane z = 0; a flat top, a quarter disk of radius 8 in
the plane z = 6; and two side flats, in the half-plane y = 0, x ≥ 0 and in the half-plane x = 0, y ≥
0, which meet along the z axis from z = 0 to z = 6 in a convex edge.

Then edit the solid directly: move each side flat 3 outward along its normal, the first to the plane
y = −3 and the second to the plane x = −3. The faces beside them follow: the sphere, the top and the
bottom run on round past the old edge to meet both moved flats, and the two flats are extended until
they meet each other, along the line x = y = −3 from z = 0 to z = 6.

So the result is the part of the slice with x ≥ −3 and y ≥ −3."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from bench import face_through
from edit import moved

RADIUS, TOP, DISTANCE = 10, 6, 3


def build():
    # the ball between the latitudes through z = 0 and z = 6, for 0° ≤ φ ≤ 90°
    piece = BRepPrimAPI_MakeSphere(RADIUS, 0, math.asin(TOP / RADIUS), math.pi / 2).Shape()
    flats = [face_through(piece, (3, 0, 1)), face_through(piece, (0, 3, 1))]
    return moved(piece, [(flat, DISTANCE) for flat in flats])
