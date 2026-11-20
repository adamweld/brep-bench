"""A block with a stepped pocket and two slotted posts standing in the pocket."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def op(kind, a, b, what):
    o = kind(a, b)
    if not o.IsDone():
        raise Refused(f"{kind.__name__} declined {what}")
    return o.Shape()


def box(x0, y0, z0, x1, y1, z1):
    return BRepPrimAPI_MakeBox(gp_Pnt(x0, y0, z0), gp_Pnt(x1, y1, z1)).Shape()


def build():
    solid = box(0, 0, 0, 20, 20, 10)
    # The pockets run on above the block's top face.
    solid = op(BRepAlgoAPI_Cut, solid, box(2, 2, 6, 18, 18, 11), "the wide pocket")
    solid = op(BRepAlgoAPI_Cut, solid, box(6, 6, 2, 14, 14, 11), "the deep pocket")
    for cx in (4, 16):
        post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(cx, 10, 6), gp.DZ_s()), 1.5, 3).Shape()
        post = op(BRepAlgoAPI_Cut, post, box(cx - 0.5, 8, 8, cx + 0.5, 12, 10), "the post's slot")
        solid = op(BRepAlgoAPI_Fuse, solid, post, "a post")
    return solid
