"""A hemisphere standing on a block, its root rounded. A block from (-30, -30, 0) to (30, 30, 10),
and standing on its top face the hemisphere of radius 8 about (0, 0, 10) above z = 10, the two
fused into one solid. The circle of radius 8 where they meet is rounded to radius 5."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from blend import edges_where, rounded

TOLERANCE = 1e-6


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-30, -30, 0), gp_Pnt(30, 30, 10)).Shape()
    dome = BRepPrimAPI_MakeSphere(gp_Ax2(gp_Pnt(0, 0, 10), gp.DZ_s()), 8, 0, math.pi / 2).Shape()
    fuse = BRepAlgoAPI_Fuse(block, dome)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the hemisphere to the block")
    part = fuse.Shape()
    root = edges_where(part, lambda p: abs(p.Z() - 10) < TOLERANCE
                       and abs(math.hypot(p.X(), p.Y()) - 8) < TOLERANCE)
    if len(root) != 1:
        raise Refused(f"{len(root)} root edges")
    return rounded(part, 5, root, "the hemisphere's root")
