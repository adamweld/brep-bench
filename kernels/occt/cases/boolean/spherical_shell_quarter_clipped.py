"""A quarter of a spherical shell, clipped by a plane. The shell between the spheres of radius 3
and 6 about the origin, where x ≥ 0, y ≥ 0 and 0 ≤ z ≤ 4."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    shell = BRepAlgoAPI_Cut(BRepPrimAPI_MakeSphere(6).Shape(), BRepPrimAPI_MakeSphere(3).Shape())
    if not shell.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the inner sphere")
    corner = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), 7, 7, 4).Shape()
    common = BRepAlgoAPI_Common(shell.Shape(), corner)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the corner block")
    return common.Shape()
