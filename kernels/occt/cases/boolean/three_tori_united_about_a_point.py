"""Three tori (major radius 3, tube radius 1) about the origin, axes z, y and x, united."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
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
    def torus(axis):
        return BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(0, 0, 0), axis), 3, 1).Shape()

    # Fuse two, then the third.
    return fuse(fuse(torus(gp.DZ_s()), torus(gp.DY_s())), torus(gp.DX_s()))
