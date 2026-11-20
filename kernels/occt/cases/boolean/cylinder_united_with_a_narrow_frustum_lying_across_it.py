"""A cylinder of radius 5, z 0 to 10, united with a frustum along the line through (0, 0, 5) parallel to x, from x = -8 (radius 3) to x = 8 (radius 1)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def fuse(first, second):
    op = BRepAlgoAPI_Fuse(first, second)
    if not op.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    # Faces the operands shared are split where they met; merge each into one.
    op.SimplifyResult()
    return op.Shape()


def cylinder(x, y, z0, radius, height):
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(x, y, z0), gp.DZ_s()), radius, height).Shape()


def build():
    # The frustum's axis runs along +x from (-8, 0, 5).
    across = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(-8, 0, 5), gp.DX_s()), 3, 1, 16).Shape()
    return fuse(cylinder(0, 0, 0, 5, 10), across)
