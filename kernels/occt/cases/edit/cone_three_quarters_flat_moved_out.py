"""Three quarters of a cone, one flat moved out past its axis. Three quarters of the cone whose
base, of radius 10, lies in the plane z = 0 centered on the origin and whose apex is (0, 0, 15):
the part whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 270°.
Its faces are the cone; the base, three quarters of a disk; and two flat triangles through the
axis, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the half-plane x = 0, y ≤ 0 (φ = 270°),
which meet along the axis, from the base's center to the apex, in a concave edge.

Then edit the solid directly: move the flat at φ = 0° by 4 outward along its normal, −y, to the
plane y = −4. The faces beside it follow it: the cone and the base run on round into the quarter
that was missing, as far as the moved flat, and the other flat, in x = 0, is trimmed back to y ≤
−4. The two flats meet along the line x = 0, y = −4, in a concave edge from the base up to the
cone at z = 9. The apex is left inside the conical face.

So the result is the whole cone less its part with x > 0 and y < −4."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp

from bench import face_through
from edit import moved

RADIUS, HEIGHT, DISTANCE = 10, 15, 4


def build():
    cone = BRepPrimAPI_MakeCone(gp.XOY_s(), RADIUS, 0, HEIGHT, 3 * math.pi / 2).Shape()
    flat = face_through(cone, (3, 0, 1))  # the half-plane y = 0, x ≥ 0
    return moved(cone, [(flat, DISTANCE)])
