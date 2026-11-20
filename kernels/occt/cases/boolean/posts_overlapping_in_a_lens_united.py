"""Two equal posts a radius apart, united. Two cylinders of radius 5 and height 10, both standing on the xy plane from z = 0 to z = 10: the first with its axis the z axis, the second with its axis the vertical line through (5, 0), a radius away. Make their union. Their discs overlap in a lens, each circle passing through the other's centre. The two tops lie in one plane, and so do the two bottoms; the walls cross along two vertical lines, at (2.5, ±2.5√3).
"""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 10).Shape()
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(5, 0, 0), gp.DZ_s()), 5, 10).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
