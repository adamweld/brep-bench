"""Two tori about z, of major radius 5 and tube radius 2, centered at z = −1 and 1, in common.
Their surfaces intersect in two separate circles; the result retains its central hole."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    lower = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, -1), gp_Dir(0, 0, 1)), 5, 2).Shape()
    upper = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 1), gp_Dir(0, 0, 1)), 5, 2).Shape()
    common = BRepAlgoAPI_Common(lower, upper)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two coaxial tori")
    return common.Shape()
