"""A half torus with the outer edge of its flat face rounded. The part with z ≥ 0 of the torus of
tube radius 2 about the circle of radius 5; the circle of radius 7 on its flat face rounded to 1/2."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus

from blend import edges_where, rounded
from bench import Refused
from edit import box


def build():
    torus = BRepPrimAPI_MakeTorus(5, 2).Shape()
    common = BRepAlgoAPI_Common(torus, box((-8, -8, 0), (8, 8, 3)))
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common could not cut the half torus")
    half = common.Shape()
    rim = edges_where(half, lambda p: abs(p.Z()) < 1e-7 and abs(math.hypot(p.X(), p.Y()) - 7) < 1e-7)
    assert len(rim) == 1, len(rim)
    return rounded(half, 0.5, rim, "the half torus's outer edge")
