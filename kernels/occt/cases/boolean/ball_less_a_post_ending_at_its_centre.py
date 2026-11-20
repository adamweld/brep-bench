"""A ball less a post that ends at its centre. A ball of radius 5 centred on the origin. A post of
radius 3 on the z axis, from z = −6 to z = 0, so its top face is the ball's equatorial section. Cut the post
away from the ball."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 5).Shape()
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -6), gp.DZ_s()), 3, 6).Shape()
    cut = BRepAlgoAPI_Cut(ball, post)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the ball and the post")
    return cut.Shape()
