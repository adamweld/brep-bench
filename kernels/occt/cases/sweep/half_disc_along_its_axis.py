"""A half disc swept along its own axis. The segment from the origin to (5, 0, 0) turned half a
turn about the z axis gives the half disc of radius 5 with y ≥ 0; sweep that face along the
straight line from the origin to (0, 0, 8)."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism, BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Pnt, gp_Vec

from bench import Refused


def build():
    segment = BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 0), gp_Pnt(5, 0, 0)).Edge()
    # a face from turning an edge half a turn: the half disc
    revol = BRepPrimAPI_MakeRevol(segment, gp.OZ_s(), math.pi)
    if not revol.IsDone():
        raise Refused("BRepPrimAPI_MakeRevol could not turn the segment into a half disc")
    sweep = BRepPrimAPI_MakePrism(revol.Shape(), gp_Vec(0, 0, 8))
    if not sweep.IsDone():
        raise Refused("BRepPrimAPI_MakePrism could not sweep the half disc")
    return sweep.Shape()
