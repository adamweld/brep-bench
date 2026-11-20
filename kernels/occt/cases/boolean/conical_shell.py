"""A conical shell. The outside is the cone rho = 8 - z/2 and the inside the cone rho = 5 - z/2,
between z = 0 and z = 8."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    outer = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 8, 4, 8).Shape()
    # The inner cone runs on past both ends, from z = -1 (radius 5.5) to z = 9 (radius 0.5).
    inner = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, -1), gp.DZ_s()), 5.5, 0.5, 10).Shape()
    cut = BRepAlgoAPI_Cut(outer, inner)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the inner cone")
    return cut.Shape()
