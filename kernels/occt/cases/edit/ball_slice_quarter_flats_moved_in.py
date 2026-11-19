"""A quarter of a slice of a ball, both its side flats moved in. The ball of radius 10 centered on
the origin, sliced by the planes z = 0 and z = 6, and a quarter of the slice: the part with x ≥ 0
and y ≥ 0, whose angle φ about the z axis lies between 0° and 90°. Its faces are the sphere; a
flat bottom, a quarter disk of radius 10 in the plane z = 0; a flat top, a quarter disk of radius
8 in the plane z = 6; and two side flats, in the half-plane y = 0, x ≥ 0 and in the half-plane x
= 0, y ≥ 0, which meet along the z axis from z = 0 to z = 6.

Then edit the solid directly: move both side flats 2 inward, along their inward normals, the
first to the plane y = 2 and the second to the plane x = 2. The faces beside them follow: the
sphere, the top and the bottom are trimmed back to meet them, and the two flats are trimmed to
meet each other along the line x = y = 2, from z = 0 to z = 6.

So the result is the part of the slice with x ≥ 2 and y ≥ 2."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from bench import face_through
from edit import moved

RADIUS, TOP, DISTANCE = 10, 6, 2


def build():
    # the ball between the latitudes through z = 0 and z = 6, for 0° ≤ φ ≤ 90°
    piece = BRepPrimAPI_MakeSphere(RADIUS, 0, math.asin(TOP / RADIUS), math.pi / 2).Shape()
    flats = [face_through(piece, (3, 0, 1)), face_through(piece, (0, 3, 1))]
    return moved(piece, [(flat, -DISTANCE) for flat in flats])  # inward
