"""A cube [0, 10]^3 with a closed cavity [2, 8]^3, united with the solid block [2, 8]^3."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse, BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
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
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    inner = BRepPrimAPI_MakeBox(gp_Pnt(2, 2, 2), gp_Pnt(8, 8, 8)).Shape()
    cut = BRepAlgoAPI_Cut(cube, inner)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cavity")
    # A solid with an inner shell, the cavity's faces looking into the void.
    filler = BRepPrimAPI_MakeBox(gp_Pnt(2, 2, 2), gp_Pnt(8, 8, 8)).Shape()
    return fuse(cut.Shape(), filler)
