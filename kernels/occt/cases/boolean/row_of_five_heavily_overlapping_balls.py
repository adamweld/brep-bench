"""A row of five heavily overlapping balls. Balls of radius 3 centred on the x axis at x = 0, 1, 2, 3
and 4, united one after another from the first."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    row = None
    for k in range(5):
        ball = BRepPrimAPI_MakeSphere(gp_Pnt(k, 0, 0), 3).Shape()
        if row is None:
            row = ball
            continue
        union = BRepAlgoAPI_Fuse(row, ball)
        if not union.IsDone():
            raise Refused(f"BRepAlgoAPI_Fuse declined the ball at x = {k}")
        row = union.Shape()
    return row
