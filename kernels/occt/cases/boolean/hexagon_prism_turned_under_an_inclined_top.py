"""A hexagonal prism turned 15°, under an inclined top. Six side planes at distance 4 from the z
axis, normals at 15° + 60°k; floor z = 0; top z = 8 + 0.25x."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from halfspace import keep_below


def build():
    shape = BRepPrimAPI_MakeBox(gp_Pnt(-8, -8, 0), 16, 16, 12).Shape()
    for k in range(6):
        a = math.radians(15 + 60 * k)
        n = (math.cos(a), math.sin(a), 0)
        shape = keep_below(shape, tuple(4 * c for c in n), n)
    return keep_below(shape, (0, 0, 8), (-0.25, 0, 1))
