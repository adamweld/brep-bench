"""A tube corner clipped by six planes. Between the cylinders of radius 3 and 5 about the z axis,
in 0 ≤ x ≤ 4, 0 ≤ y ≤ 4, from z = 0 up to the plane z = 6 + x/2."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from halfspace import keep_below


def build():
    outer = BRepPrimAPI_MakeCylinder(5, 10).Shape()
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -1), gp.DZ_s()), 3, 12).Shape()
    tube = BRepAlgoAPI_Cut(outer, bore)
    if not tube.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the bore")
    square = BRepPrimAPI_MakeBox(4, 4, 10).Shape()
    corner = BRepAlgoAPI_Common(tube.Shape(), square)
    if not corner.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the square block")
    # z − x/2 ≤ 6: the plane through (0, 0, 6), its normal (−0.5, 0, 1).
    return keep_below(corner.Shape(), (0, 0, 6), (-0.5, 0, 1))
