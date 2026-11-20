"""A cylinder of radius 5, z 0 to 6, united with a coaxial frustum from z = -3 (radius 3) to z = 9 (radius 3/2)."""
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


def frustum(x, y, z0, r0, r1, height):
    """Radius r0 at z0, r1 at z0 + height."""
    return BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(x, y, z0), gp.DZ_s()), r0, r1, height).Shape()


def build():
    return fuse(cylinder(0, 0, 0, 5, 6), frustum(0, 0, -3, 3, 1.5, 12))
