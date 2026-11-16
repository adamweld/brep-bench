"""A ball of radius 5 about the origin, its top cut flat at z = 3, the flat deleted: the ball closes
over it.

The body is cut by BRepAlgoAPI_Cut, and BRepAlgoAPI_Defeaturing deletes the faces, picked by a
point inside each."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import face_through
from edit import box, deleted


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 5).Shape()
    body = BRepAlgoAPI_Cut(ball, box((-6, -6, 3), (6, 6, 6))).Shape()
    return deleted(body, [face_through(body, (1, 0, 3))])
