"""Cutting a solid by a plane, as OCCT's users do: Boolean-cut it with a block laid on the plane
and reaching well past the solid on the side to be removed."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused

REACH = 500


def keep_below(shape, point, normal):
    """`shape` where (p − point)·normal ≤ 0: the part on the side the normal points away from."""
    ax = gp_Ax2(gp_Pnt(*point), gp_Dir(*normal))
    x, y = ax.XDirection(), ax.YDirection()
    corner = gp_Pnt(point[0] - REACH * (x.X() + y.X()), point[1] - REACH * (x.Y() + y.Y()),
                    point[2] - REACH * (x.Z() + y.Z()))
    block = BRepPrimAPI_MakeBox(gp_Ax2(corner, ax.Direction(), x), 2 * REACH, 2 * REACH, REACH).Shape()
    cut = BRepAlgoAPI_Cut(shape, block)
    if not cut.IsDone():
        raise Refused(f"BRepAlgoAPI_Cut declined the plane through {point}")
    return cut.Shape()
