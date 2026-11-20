"""A wall bracket: a base plate, an upright, two ribs, a lug on top, a boss on the upright's face,
two bolt holes in the base and a hole through the boss and the upright."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def op(kind, a, b, what):
    o = kind(a, b)
    if not o.IsDone():
        raise Refused(f"{kind.__name__} declined {what}")
    return o.Shape()


def box(x0, y0, z0, x1, y1, z1):
    return BRepPrimAPI_MakeBox(gp_Pnt(x0, y0, z0), gp_Pnt(x1, y1, z1)).Shape()


def build():
    solid = box(0, 0, 0, 20, 12, 2)
    for part in (box(0, 0, 2, 20, 2, 14), box(0, 2, 2, 2, 8, 6), box(18, 2, 2, 20, 8, 6),
                 box(7, 0, 14, 13, 2, 16),
                 BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(10, 2, 10), gp_Dir(0, 1, 0)), 3, 2).Shape()):
        solid = op(BRepAlgoAPI_Fuse, solid, part, "a part")
    # The holes run past the faces they open in.
    for cx in (5, 15):
        bolt = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(cx, 8, -1), gp.DZ_s()), 1, 4).Shape()
        solid = op(BRepAlgoAPI_Cut, solid, bolt, "a bolt hole")
    hole = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(10, -1, 10), gp_Dir(0, 1, 0)), 1.5, 6).Shape()
    return op(BRepAlgoAPI_Cut, solid, hole, "the hole through the boss")
