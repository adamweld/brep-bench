"""A drum domed by a ball whose poles lie on its rim. The cylinder of radius 5 on the z axis from
z = 0 to z = 8, united with the ball of radius 5 about (0, 0, 8), the middle of its top, the ball's
poles at (±5, 0, 8) on the drum's top rim, and its seam the half of the rim with y > 0."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    drum = BRepPrimAPI_MakeCylinder(5, 8).Shape()
    # the sphere's poles lie on its placement's main direction, its seam toward its x direction
    ball = BRepPrimAPI_MakeSphere(gp_Ax2(gp_Pnt(0, 0, 8), gp.DX_s(), gp.DY_s()), 5).Shape()
    fuse = BRepAlgoAPI_Fuse(drum, ball)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the drum and the ball")
    return fuse.Shape()
