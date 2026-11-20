"""A cone's end face drafted about a diameter, the pull across the axis. A frustum about the z
axis, radius 10 at z = 0 narrowing at a half-angle of 15° to z = 20, its top face drafted 10° with
the pull direction +x and the neutral plane x = 0: it turns about the diameter x = 0, z = 20 into
z = 20 + x·tan 10°."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

from bench import face_through
from draft import drafted


def build():
    top_radius = 10 - 20 * math.tan(math.radians(15))
    frustum = BRepPrimAPI_MakeCone(10, top_radius, 20).Shape()
    top = face_through(frustum, (0, 0, 20))
    return drafted(frustum, [top], 10, neutral=(0, 0, 20), pull=(1, 0, 0))
