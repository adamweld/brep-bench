"""Bullet A united with bullet B, crossing at right angles off its axis. A bullet is the profile (0, 0),
(6, 0), (6, 8), (3, 14), (0, 14) turned about its axis. A stands on the xy plane about the line
through (0, 1.5); B is A turned 180° about the line through (0, 0, 7) in the direction (1, 0, 1)."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon, BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt, gp_Trsf

from bench import Refused

PROFILE = [(0, 0), (6, 0), (6, 8), (3, 14), (0, 14)]  # (distance from the axis, along it)


def bullet():
    """Bullet A: the profile in the plane y = 1.5, turned about the vertical line x = 0, y = 1.5."""
    polygon = BRepBuilderAPI_MakePolygon()
    for r, z in PROFILE:
        polygon.Add(gp_Pnt(r, 1.5, z))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire()).Face()
    return BRepPrimAPI_MakeRevol(face, gp_Ax1(gp_Pnt(0, 1.5, 0), gp_Dir(0, 0, 1)), 2 * math.pi).Shape()


def build():
    a = bullet()
    turn = gp_Trsf()
    turn.SetRotation(gp_Ax1(gp_Pnt(0, 0, 7), gp_Dir(1, 0, 1)), math.pi)
    b = BRepBuilderAPI_Transform(a, turn, True).Shape()
    op = BRepAlgoAPI_Fuse(a, b)
    if not op.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two bullets")
    return op.Shape()
