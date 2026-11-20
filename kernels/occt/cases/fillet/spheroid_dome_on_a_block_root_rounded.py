"""A spheroid dome standing on a block, its root rounded. A block from (-30, -30, 0) to
(30, 30, 10), and standing on its top face the half above z = 10 of the spheroid about the z axis
with centre (0, 0, 10), radius 8 across and 12 up, the two fused into one solid. The circle of
radius 8 where they meet is rounded to radius 2."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax1, gp_Ax2, gp_Dir, gp_Elips, gp_Pnt

from bench import Refused
from blend import edges_where, rounded

TOLERANCE = 1e-6


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-30, -30, 0), gp_Pnt(30, 30, 10)).Shape()
    # The half ellipse in the plane y = 0: major radius 12 along z, minor 8 along x, from the
    # top (0, 0, 22) at parameter 0 down to the equator (8, 0, 10) at pi / 2.
    ellipse = gp_Elips(gp_Ax2(gp_Pnt(0, 0, 10), gp_Dir(0, 1, 0), gp_Dir(0, 0, 1)), 12, 8)
    arc = BRepBuilderAPI_MakeEdge(ellipse, 0, math.pi / 2).Edge()
    floor = BRepBuilderAPI_MakeEdge(gp_Pnt(8, 0, 10), gp_Pnt(0, 0, 10)).Edge()
    axis = BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 10), gp_Pnt(0, 0, 22)).Edge()
    wire = BRepBuilderAPI_MakeWire(arc, floor, axis).Wire()
    dome = BRepPrimAPI_MakeRevol(BRepBuilderAPI_MakeFace(wire).Face(),
                                 gp_Ax1(gp_Pnt(0, 0, 10), gp.DZ_s())).Shape()
    fuse = BRepAlgoAPI_Fuse(block, dome)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the dome to the block")
    part = fuse.Shape()
    root = edges_where(part, lambda p: abs(p.Z() - 10) < TOLERANCE
                       and abs(math.hypot(p.X(), p.Y()) - 8) < TOLERANCE)
    if len(root) != 1:
        raise Refused(f"{len(root)} root edges")
    return rounded(part, 2, root, "the dome's root")
