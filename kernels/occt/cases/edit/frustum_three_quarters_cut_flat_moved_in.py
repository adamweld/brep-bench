"""Three quarters of a cone frustum, one cut flat moved in. Three quarters of a cone frustum about the z axis, of radius 9 at z = 0 and 3 at z = 12 (its cone's
apex would be at z = 18): the part whose angle about the z axis, φ, measured from +x toward +y,
lies between 0° and 270°. Its faces are the cone; a bottom and a top, three quarters of disks of
radius 9 and 3; and two flat trapezoids, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the
half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the axis in a concave edge.

Then edit the solid directly: move the flat at φ = 0° 2 inward, against its outward normal −y, to
the plane y = 2. The flat gives up the slab it stood on, so the faces beside it are trimmed: the
cone, the bottom and the top lose their parts with x > 0 and y < 2, and the other flat, in x = 0,
runs on to y ≤ 2. The two flats meet along the line x = 0, y = 2, in a concave edge from the bottom
to the top. The axis itself is left in the part taken away.

So the result is the whole frustum less its part with x > 0 and y < 2."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp

from bench import face_through
from edit import moved

DISTANCE = 2


def build():
    frustum = BRepPrimAPI_MakeCone(gp.XOY_s(), 9, 3, 12, 3 * math.pi / 2).Shape()
    flat = face_through(frustum, (5, 0, 3))  # the half-plane y = 0, x >= 0; its normal is -y
    return moved(frustum, [(flat, -DISTANCE)])
