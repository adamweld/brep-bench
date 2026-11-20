"""Half a conical shell. The part with x ≥ 0, from z = 0 to z = 10, between a cone of radius 6 at
z = 0 and 3 at z = 10 and a coaxial cone of radius 4 at z = 0 and 1 at z = 10."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from halfspace import keep_below


def build():
    outer = BRepPrimAPI_MakeCone(6, 3, 10).Shape()
    # The inner cone continues the same slope (0.3 per unit) a unit past each end: 4.3 at z = −1
    # to 0.7 at z = 11.
    inner = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, -1), gp.DZ_s()), 4.3, 0.7, 12).Shape()
    shell = BRepAlgoAPI_Cut(outer, inner)
    if not shell.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the inner cone")
    return keep_below(shell.Shape(), (0, 0, 0), (-1, 0, 0))
