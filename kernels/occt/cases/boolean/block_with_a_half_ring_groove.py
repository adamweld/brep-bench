"""A block with a half ring cut through it from its face. The block from (-12, 0, 0) to (12, 12, 12), less the ring made by
turning about the z axis the stadium in the plane y = 0 between x = 6 and x = 10, from z = 3 to z = 9, with a half disk of
radius 2 at each end."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.GC import GC_MakeArcOfCircle
from OCP.gp import gp, gp_Ax1, gp_Pnt

from bench import Refused


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(-12, 0, 0), gp_Pnt(12, 12, 12)).Shape()
    # the profile in the plane y = 0: straight sides x = 6 and x = 10, z from 3 to 9, half disks of radius 2 at the ends
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(6, 0, 3), gp_Pnt(6, 0, 9)).Edge())
    top = GC_MakeArcOfCircle(gp_Pnt(6, 0, 9), gp_Pnt(8, 0, 11), gp_Pnt(10, 0, 9)).Value()
    wire.Add(BRepBuilderAPI_MakeEdge(top).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(10, 0, 9), gp_Pnt(10, 0, 3)).Edge())
    bottom = GC_MakeArcOfCircle(gp_Pnt(10, 0, 3), gp_Pnt(8, 0, 1), gp_Pnt(6, 0, 3)).Value()
    wire.Add(BRepBuilderAPI_MakeEdge(bottom).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    ring = BRepPrimAPI_MakeRevol(face, gp_Ax1(gp_Pnt(0, 0, 0), gp.DZ_s()), 2 * math.pi).Shape()
    cut = BRepAlgoAPI_Cut(block, ring)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the block and the ring")
    return cut.Shape()
