"""A frustum z 0 to 5 (radius 3 to 2) united with a frustum z -4 to 0 (radius 1 to 3) under it."""
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
    return fuse(frustum(0, 0, 0, 3, 2, 5), frustum(0, 0, -4, 1, 3, 4))
