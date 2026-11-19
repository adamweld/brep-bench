"""Three quarters of a cut ball shelled inward, open at both its flats. Three quarters of the ball
of radius 5 centred on the origin, cut flat at z = 3: the part with z ≤ 3 whose angle about the z
axis, φ, measured from +x toward +y, lies between 0° and 270°. It runs from the south pole,
(0, 0, −5), which is on the z axis, up to the plane z = 3. Its faces are the sphere, the flat top
(three quarters of the disk of radius 4 at z = 3), and two flat faces in the half-plane y = 0,
x ≥ 0 (φ = 0°) and in the half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the z axis at a
concave edge.

Shell it inward to a wall 1 thick, open at both faces in the half-planes. The outside of the wall
is the sphere and the top, unmoved, and the inside is each of them moved 1 inward along its normal:
the sphere onto the sphere of radius 4 about the origin, and the top onto the plane z = 2. The
moved faces are trimmed where they meet each other, and run on until they meet the surfaces of the
openings, the two half-planes, which are left as rims between the outside and the inside.

So the result is the cut ball's three quarters less the cavity, the part of the ball of radius 4
below z = 2 with φ between 0° and 270°. The cavity's flat is three quarters of the disk of radius
√12 at z = 2. Each rim is the part of its half-plane between the two: inside the circle of radius
5 and below z = 3, outside the circle of radius 4 or above z = 2. The two rims meet along the z
axis from (0, 0, −5) to (0, 0, −4) and from (0, 0, 2) to (0, 0, 3)."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from shell import face_through, shell

RADIUS, TOP, TURN = 5, 3, 270  # the wedge's angle about z, degrees, from +x
THICKNESS = 1


def build():
    # from the south pole (latitude −90°) up to the latitude of z = 3, poles on the z axis
    wedge = BRepPrimAPI_MakeSphere(RADIUS, -math.pi / 2, math.asin(TOP / RADIUS),
                                   math.radians(TURN)).Shape()
    flats = [face_through(wedge, (2, 0, 1)), face_through(wedge, (0, -2, 1))]
    return shell(wedge, flats, THICKNESS)
