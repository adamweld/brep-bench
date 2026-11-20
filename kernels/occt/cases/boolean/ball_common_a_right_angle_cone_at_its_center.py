"""A ball in common with a right-angle cone at its center. A ball of radius 5 about the origin; a
solid cone with its apex at the origin, axis +z, half-angle 45°, up to z = 6 (base radius 6). Make the
part common to both."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    ball = BRepPrimAPI_MakeSphere(gp_Pnt(0, 0, 0), 5).Shape()
    cone = BRepPrimAPI_MakeCone(0, 6, 6).Shape()  # apex radius 0 at z = 0, radius 6 at z = 6
    common = BRepAlgoAPI_Common(ball, cone)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the ball and the cone at its center")
    return common.Shape()
