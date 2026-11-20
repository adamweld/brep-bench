"""A cube united with a longer cylinder whose axis is one of its vertical edges. A cube with x, y and z from 0 to 10. A cylinder of radius 6, its axis the z axis, which is the cube's vertical edge through the origin, from z = −3 to z = 13: it runs 3 below the cube and 3 above it. Make the union of the cube and the cylinder."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax1, gp_Ax2, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused



def build():
    first = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    second = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -3), gp.DZ_s()), 6, 16).Shape()
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two solids")
    return fuse.Shape()
