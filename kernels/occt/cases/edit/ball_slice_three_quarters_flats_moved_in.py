"""Three quarters of a slice of a ball, both its side flats moved in. The ball of radius 10 centered on
the origin, sliced by the planes z = 0 and z = 6, and three quarters of the slice: the part whose
angle φ about the z axis, measured from +x toward +y, lies between 0° and 270°. Its faces are the
sphere; a flat bottom, three quarters of a disk of radius 10 in the plane z = 0; a flat top, three
quarters of a disk of radius 8 in the plane z = 6; and two side flats, in the half-plane y = 0, x ≥
0 (φ = 0°) and in the half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the z axis from z = 0 to
z = 6 in a concave edge.

Then edit the solid directly: move each side flat 3 inward along its inward normal, the first, at φ
= 0°, to the plane y = 3 and the second, at φ = 270°, to the plane x = −3. The faces beside them
follow: the sphere, the top and the bottom are trimmed back to meet the moved flats, and the two
flats are extended until they meet each other, along the line x = −3, y = 3 from z = 0 to z = 6, in
a concave edge.

So the result is the part of the slice with x ≤ −3 or y ≥ 3."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from bench import face_through
from edit import moved

RADIUS, TOP, DISTANCE = 10, 6, 3


def build():
    # the ball between the latitudes through z = 0 and z = 6, for 0° ≤ φ ≤ 270°
    piece = BRepPrimAPI_MakeSphere(RADIUS, 0, math.asin(TOP / RADIUS), 3 * math.pi / 2).Shape()
    flats = [face_through(piece, (3, 0, 1)), face_through(piece, (0, -3, 1))]
    return moved(piece, [(flat, -DISTANCE) for flat in flats])  # inward
