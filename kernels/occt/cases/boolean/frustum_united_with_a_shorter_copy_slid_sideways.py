"""A frustum z 0 to 8 (radius 6 to 2) united with a shorter piece of the same cone, z 0 to 5 (radius 6 to 3.5), on the axis through (4, 0)."""
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
    # Radius is 6 - z/2 on both: the second is the same cone cut at z = 5, axis moved 4 along x.
    return fuse(frustum(0, 0, 0, 6, 2, 8), frustum(4, 0, 0, 6, 3.5, 5))
