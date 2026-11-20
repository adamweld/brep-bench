"""A cylinder and a narrower cylinder crossing beside its axis and touching its wall at a point, their common part. A cylinder of radius 4 along z from -6 to 6, common with a cylinder of radius 2 along x, its axis at y = 2."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    thick = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -6), gp.DZ_s()), 4, 12).Shape()
    thin = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(-6, 2, 0), gp.DX_s()), 2, 12).Shape()
    result = BRepAlgoAPI_Common(thick, thin)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the case")
    return result.Shape()
