"""A post of radius 4, z 0 to 8, united with a torus about its axis at z = 4, major radius 4, tube radius 3/2."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder, BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def fuse(first, second):
    op = BRepAlgoAPI_Fuse(first, second)
    if not op.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    # Faces the operands shared are split where they met; merge each into one.
    op.SimplifyResult()
    return op.Shape()


def build():
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 4, 8).Shape()
    ring = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 4), gp.DZ_s()), 4, 1.5).Shape()
    return fuse(post, ring)
