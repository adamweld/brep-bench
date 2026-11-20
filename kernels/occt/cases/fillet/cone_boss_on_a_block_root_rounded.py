"""A cone frustum standing on a block, its root rounded. A block from (-20, -20, 0) to (20, 20, 10),
and standing on its top face a cone frustum about the z axis, radius 9 at z = 10 and radius 3 at
z = 18, the two fused into one solid. The circle of radius 9 where the cone meets the top face is
rounded to radius 2."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from blend import edges_where, rounded

TOLERANCE = 1e-6


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-20, -20, 0), gp_Pnt(20, 20, 10)).Shape()
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 10), gp.DZ_s()), 9, 3, 8).Shape()
    fuse = BRepAlgoAPI_Fuse(block, frustum)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the frustum to the block")
    part = fuse.Shape()
    root = edges_where(part, lambda p: abs(p.Z() - 10) < TOLERANCE
                       and abs(math.hypot(p.X(), p.Y()) - 9) < TOLERANCE)
    if len(root) != 1:
        raise Refused(f"{len(root)} root edges")
    return rounded(part, 2, root, "the frustum's root")
