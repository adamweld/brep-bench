"""The common part of four radius-10 balls at the vertices of a regular tetrahedron of side 10.
The six circular edges are kept, not rounded into a Meissner body."""
from math import sqrt

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused

CENTERS = [(0, 0, 0), (10, 0, 0), (5, 5 * sqrt(3), 0),
           (5, 5 * sqrt(3) / 3, 10 * sqrt(2 / 3))]
RADIUS = 10


def build():
    solid = BRepPrimAPI_MakeSphere(gp_Pnt(*CENTERS[0]), RADIUS).Shape()
    for center in CENTERS[1:]:
        ball = BRepPrimAPI_MakeSphere(gp_Pnt(*center), RADIUS).Shape()
        common = BRepAlgoAPI_Common(solid, ball)
        if not common.IsDone():
            raise Refused(f"BRepAlgoAPI_Common declined the ball at {center}")
        solid = common.Shape()
    return solid
