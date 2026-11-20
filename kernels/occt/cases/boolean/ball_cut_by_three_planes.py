"""A ball cut by three planes. A ball of radius 5 about the origin, kept where x ≥ 0 and
−3 ≤ z ≤ 4."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from halfspace import keep_below


def build():
    ball = BRepPrimAPI_MakeSphere(5).Shape()
    ball = keep_below(ball, (0, 0, 0), (-1, 0, 0))
    ball = keep_below(ball, (0, 0, -3), (0, 0, -1))
    return keep_below(ball, (0, 0, 4), (0, 0, 1))
