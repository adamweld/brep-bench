"""A chain of three balls. Balls of radius 5, 4 and 3 centred on the origin, (7, 0, 0) and
(10, 4, 0). Unite the first two, then unite the third with that."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused

BALLS = [((0, 0, 0), 5), ((7, 0, 0), 4), ((10, 4, 0), 3)]


def build():
    chain = None
    for centre, radius in BALLS:
        ball = BRepPrimAPI_MakeSphere(gp_Pnt(*centre), radius).Shape()
        if chain is None:
            chain = ball
            continue
        union = BRepAlgoAPI_Fuse(chain, ball)
        if not union.IsDone():
            raise Refused(f"BRepAlgoAPI_Fuse declined the ball of radius {radius}")
        chain = union.Shape()
    return chain
