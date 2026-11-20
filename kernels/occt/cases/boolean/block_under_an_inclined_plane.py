"""A block under an inclined plane. Above z = 0, between y = 0 and y = 6, between x = 0 and
x = 10, below the plane z = 2 + 0.3x."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from halfspace import keep_below


def build():
    block = BRepPrimAPI_MakeBox(10, 6, 6).Shape()
    # z − 0.3x ≤ 2: the plane through (0, 0, 2), its normal (−0.3, 0, 1).
    return keep_below(block, (0, 0, 2), (-0.3, 0, 1))
