"""A torus less the post its core circle lies on. A post of radius 4 on the z axis from z = −3 to
z = 3. A torus about the z axis centred at the origin: core circle of radius 4, tube radius 1. Cut the post
away from the torus."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -3), gp.DZ_s()), 4, 6).Shape()
    torus = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 4, 1).Shape()
    cut = BRepAlgoAPI_Cut(torus, post)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the torus and the post")
    return cut.Shape()
