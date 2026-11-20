"""Three quarters of a cone. In the xz plane, take the right triangle with corners (0, 0),
(6, 0) and (0, 6). Revolve it 270° about the z axis, right-handed: from +x toward +y."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone

RADIUS, HEIGHT = 6, 6
ANGLE = 270  # degrees


def build():
    # Base on z = 0, apex at z = HEIGHT, swept from +x toward +y.
    return BRepPrimAPI_MakeCone(RADIUS, 0, HEIGHT, math.radians(ANGLE)).Shape()
