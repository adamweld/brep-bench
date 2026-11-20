"""A thick cylinder with a tilted thin cylinder cut into its side off-centre. A thick cylinder of
radius 5 about the z axis, z from 0 to 10, less the cylinder of radius 1 and length 9 from (1, -7, 7)
along (0, cos 30, -sin 30)."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    thick = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 10).Shape()
    c = math.cos(math.radians(30))
    s = math.sin(math.radians(30))
    thin = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(1, -7, 7), gp_Dir(0, c, -s)), 1, 9).Shape()
    cut = BRepAlgoAPI_Cut(thick, thin)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the tilted hole")
    return cut.Shape()
