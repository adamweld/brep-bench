"""Three quarters of a cone frustum, both its ends moved out. Three quarters of a cone frustum about
the z axis, of radius 10 at z = 0 and 4 at z = 12 (its cone's apex would be at z = 20): the part
whose angle about the z axis, φ, measured from +x toward +y, lies between 0° and 270°. Its faces
are the cone; a bottom and a top, three quarters of disks of radius 10 and 4; and two flat
trapezoids, in the half-planes φ = 0° and φ = 270°, which meet along the axis in a concave edge.

Then edit the solid directly: move the top 4 up and the bottom 3 down, each along its outward
normal, to the planes z = 16 and z = −3. The cone and the two flats run on to meet them, so the
top shrinks to radius 2 and the bottom grows to radius 11.5.

So the result is three quarters of the frustum of the same cone from z = −3, radius 11.5, to z =
16, radius 2."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp

from bench import face_through
from edit import moved


def build():
    frustum = BRepPrimAPI_MakeCone(gp.XOY_s(), 10, 4, 12, 3 * math.pi / 2).Shape()
    top = face_through(frustum, (-1, 1, 12))
    bottom = face_through(frustum, (-1, 1, 0))
    return moved(frustum, [(top, 4), (bottom, 3)])
