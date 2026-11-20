"""A block from (-10, -10, 0) to (10, 10, 5) united with a thin slanting ring: the parallelogram
(rho, z) = (3, 2), (4, 2), (5, 9), (4, 9) turned about the z axis."""
from math import pi

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax1, gp_Pnt

from bench import Refused


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, 0), gp_Pnt(10, 10, 5)).Shape()
    section = BRepBuilderAPI_MakePolygon()
    for rho, z in [(3, 2), (4, 2), (5, 9), (4, 9)]:
        section.Add(gp_Pnt(rho, 0, z))
    section.Close()
    face = BRepBuilderAPI_MakeFace(section.Wire()).Face()
    ring = BRepPrimAPI_MakeRevol(face, gp_Ax1(gp_Pnt(0, 0, 0), gp.DZ_s()), 2 * pi).Shape()
    fuse = BRepAlgoAPI_Fuse(block, ring)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the block and the ring")
    fuse.SimplifyResult()
    return fuse.Shape()
