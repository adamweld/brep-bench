"""A half cylinder bored off-axis across its flat. A cylinder of radius 5 and height 8 on the z
axis, split along y = 0 (keep y ≥ 0), with a cylinder of radius 2 on the axis through (0, 1)
bored through, past both ends."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from halfspace import keep_below


def build():
    cylinder = BRepPrimAPI_MakeCylinder(5, 8).Shape()
    half = keep_below(cylinder, (0, 0, 0), (0, -1, 0))
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 1, -1), gp.DZ_s()), 2, 10).Shape()
    cut = BRepAlgoAPI_Cut(half, bore)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the bore")
    return cut.Shape()
