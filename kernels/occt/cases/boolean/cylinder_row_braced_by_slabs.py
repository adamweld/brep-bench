"""A row of three cylinders braced by four stadium-shaped slabs, all fused."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def cyl(x, z, radius, height):
    return BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(x, 0, z), gp.DZ_s()), radius, height).Shape()


def build():
    solid = cyl(0, 0, 2, 9)
    parts = [cyl(5, 0, 2, 9), cyl(10, 0, 2, 9)]
    for z in (1, 3, 5, 7):
        # A slab: a box between the end centres with a half-disk (a whole disk, whose other half
        # is inside the box) on each end.
        parts.append(BRepPrimAPI_MakeBox(gp_Pnt(0, -1, z), gp_Pnt(10, 1, z + 1)).Shape())
        parts.append(cyl(0, z, 1, 1))
        parts.append(cyl(10, z, 1, 1))
    for part in parts:
        fuse = BRepAlgoAPI_Fuse(solid, part)
        if not fuse.IsDone():
            raise Refused("BRepAlgoAPI_Fuse declined a part")
        solid = fuse.Shape()
    return solid
