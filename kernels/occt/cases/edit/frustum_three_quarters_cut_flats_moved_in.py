"""Three quarters of a cone frustum, both cut flats moved in. Three quarters of a cone frustum about the z axis, of radius 9 at z = 0 and 3 at z = 12 (its cone's
apex would be at z = 18): the part whose angle about the z axis, φ, measured from +x toward +y,
lies between 0° and 270°. Its faces are the cone; a bottom and a top, three quarters of disks of
radius 9 and 3; and two flat trapezoids, in the half-plane y = 0, x ≥ 0 (φ = 0°) and in the
half-plane x = 0, y ≤ 0 (φ = 270°), which meet along the axis in a concave edge.

Then edit the solid directly: move each of the two flats 2 inward, against its outward normal: the
one at φ = 0° from y = 0 to the plane y = 2, and the one at φ = 270°, whose outward normal is +x,
from x = 0 to the plane x = −2. The faces beside them are trimmed, and each flat runs on to meet the
other, in a concave edge along the line x = −2, y = 2 from the bottom to the top. The cone, the
bottom and the top lose the parts they had with x > −2 and y < 2.

So the result is the whole frustum less its part with x > −2 and y < 2."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp

from bench import face_through
from edit import moved

DISTANCE = 2


def build():
    frustum = BRepPrimAPI_MakeCone(gp.XOY_s(), 9, 3, 12, 3 * math.pi / 2).Shape()
    first = face_through(frustum, (5, 0, 3))    # y = 0, x >= 0: its outward normal is -y
    second = face_through(frustum, (0, -5, 3))  # x = 0, y <= 0: its outward normal is +x
    return moved(frustum, [(first, -DISTANCE), (second, -DISTANCE)])
