"""Three quarters of a ball shelled outward, open at both its flats. Three quarters of the ball of
radius 5 centred on the origin, from pole to pole: the part whose angle about the z axis, φ,
measured from +x toward +y, lies between 0° and 270°. Its poles are on the z axis, at (0, 0, ±5).
Its faces are the sphere and two flat half disks, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in
the half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the z axis at a concave edge.

Shell it outward to a wall 1 thick, open at both flat faces. The inside of the wall is the sphere,
unmoved, and the outside is the sphere moved 1 outward along its normal, onto the sphere of radius
6. The moved face runs on until it meets the surfaces of the openings, the two half-planes, which
are left as rims between the outside and the inside.

So the result is the three quarters of the spherical shell between radii 5 and 6 with φ between 0°
and 270°, its rims the two half annuli between those radii in the two half-planes. The rims meet
each other along the z axis from (0, 0, 5) to (0, 0, 6) and from (0, 0, −5) to (0, 0, −6), through
the poles of both spheres."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from shell import face_through, shell

RADIUS, TURN = 5, 270  # the wedge's angle about z, degrees, from +x
THICKNESS = 1


def build():
    # the sphere's own poles on the z axis, its meridian at φ = 0 on +x
    wedge = BRepPrimAPI_MakeSphere(RADIUS, -math.pi / 2, math.pi / 2, math.radians(TURN)).Shape()
    flats = [face_through(wedge, (2, 0, 1)), face_through(wedge, (0, -2, 1))]
    return shell(wedge, flats, THICKNESS, outward=True)
