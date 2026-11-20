"""Four slabs united with a post across their joints. Four 10 by 10 by 2 slabs, z from 0 to 2, tiling
[0,20]^2, united with the cylinder of radius 6 about the line x = y = 10 from z = -1 to z = 5."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    shape = None
    for x in (0, 10):
        for y in (0, 10):
            slab = BRepPrimAPI_MakeBox(gp_Pnt(x, y, 0), gp_Pnt(x + 10, y + 10, 2)).Shape()
            if shape is None:
                shape = slab
                continue
            fuse = BRepAlgoAPI_Fuse(shape, slab)
            if not fuse.IsDone():
                raise Refused("BRepAlgoAPI_Fuse declined two slabs")
            shape = fuse.Shape()
    post = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(10, 10, -1), gp.DZ_s()), 6, 6).Shape()
    fuse = BRepAlgoAPI_Fuse(shape, post)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the post")
    return fuse.Shape()
