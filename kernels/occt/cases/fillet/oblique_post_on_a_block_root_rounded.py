"""A leaning cylinder standing on a block, its root rounded. A block from (-30, -30, 0) to
(30, 30, 10), and a cylinder of radius 5 about the line through (0, 0, 10) leaning 30 degrees from
the vertical toward +x, from axial distance -5 (inside the block) to a flat end at 20, fused into
one solid. The ellipse where it meets the block's top is rounded to radius 1.5."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from blend import edges_where, rounded

LEAN = math.radians(30)


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-30, -30, 0), gp_Pnt(30, 30, 10)).Shape()
    axis = gp_Dir(math.sin(LEAN), 0, math.cos(LEAN))
    start = gp_Pnt(-5 * math.sin(LEAN), 0, 10 - 5 * math.cos(LEAN))
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(start, axis), 5, 25).Shape()
    fuse = BRepAlgoAPI_Fuse(block, post)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the post to the block")
    part = fuse.Shape()
    # The junction is the only edge in the plane z = 10 away from the block's own rim.
    root = edges_where(part, lambda p: abs(p.Z() - 10) < 1e-6 and abs(p.X()) < 10 and abs(p.Y()) < 10)
    if len(root) != 1:
        raise Refused(f"{len(root)} junction edges")
    return rounded(part, 1.5, root, "the leaning post's root")
