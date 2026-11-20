"""A ball less an octant, the ball turned off the axes. A ball of radius 5 centered on the origin,
bounded by a plain spherical surface whose axis runs along (1, 2, 2)/3 and whose seam lies in the
half-plane through that axis and the direction (2, 1, -2)/3. Cut from it the box from (0, 0, 0) to
(8, 8, 8)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    frame = gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(1, 2, 2), gp_Dir(2, 1, -2))
    ball = BRepPrimAPI_MakeSphere(frame, 5).Shape()
    box = BRepPrimAPI_MakeBox(8, 8, 8).Shape()
    cut = BRepAlgoAPI_Cut(ball, box)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the ball and the box")
    return cut.Shape()
