"""A post united with a square block whose sides are inside the post and whose corners poke out, flush at both ends. A post: a solid cylinder of radius 5 standing on the xy plane, its axis the z axis, from z = 0 to z = 10. A block of square section, x and y from −4 to 4, and z from 0 to 10: it is as tall as the post, its top and bottom faces level with the post's. Make the union of the post and the block."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax1, gp_Ax2, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused

LO, HI = (-4, -4, 0), (4, 4, 10)


def build():
    first = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, 0), gp.DZ_s()), 5, 10).Shape()
    second = BRepPrimAPI_MakeBox(gp_Pnt(*LO), gp_Pnt(*HI)).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    # The fuse leaves the flush ends split where the solids met; merge each into one face.
    fuse.SimplifyResult()
    return fuse.Shape()
