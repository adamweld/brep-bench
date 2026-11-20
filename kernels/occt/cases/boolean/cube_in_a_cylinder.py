"""A cube in a cylinder. The cube from (-4, -4, 0) to (4, 4, 8), common with the cylinder of
radius 5 on the z axis from z = -2 to z = 10."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(-4, -4, 0), gp_Pnt(4, 4, 8)).Shape()
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -2), gp.DZ_s()), 5, 12).Shape()
    common = BRepAlgoAPI_Common(cube, cylinder)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and cylinder")
    return common.Shape()
