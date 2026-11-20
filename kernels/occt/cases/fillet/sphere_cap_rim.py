"""A sphere cap with its rim rounded. The part of the ball of radius 5 about the origin with
z ≥ 3, its circular edge at z = 3 rounded to radius 1/2."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere

from blend import edges_where, rounded
from bench import Refused
from edit import box


def build():
    ball = BRepPrimAPI_MakeSphere(5).Shape()
    common = BRepAlgoAPI_Common(ball, box((-6, -6, 3), (6, 6, 6)))
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common could not cut the cap from the ball")
    cap = common.Shape()
    rim = edges_where(cap, lambda p: abs(p.Z() - 3) < 1e-7
                      and abs(math.hypot(p.X(), p.Y()) - 4) < 1e-7)
    assert len(rim) == 1, len(rim)
    return rounded(cap, 0.5, rim, "the cap's rim")
