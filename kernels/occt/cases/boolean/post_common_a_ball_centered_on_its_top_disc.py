"""A post in common with a ball centered on its top disc. A post of radius 4 on the z axis from z = 0
to z = 10, and a ball of radius 2 centered at (0, 0, 10). Make the part common to both."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(4, 10).Shape()
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 10), 2).Shape()
    common = BRepAlgoAPI_Common(post, ball)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the post and the ball")
    return common.Shape()
