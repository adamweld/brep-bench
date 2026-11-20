"""A tube round an ellipse, hollowed and cut open by a plane. Ellipse in the xy plane, semi-axes 20
(x) and 12 (y). A circle of radius 3 swept along it, less one of radius 2, cut by x <= 15."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax2, gp_Circ, gp_Dir, gp_Elips, gp_Pnt

from bench import Refused


def tube(radius):
    path = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(gp_Elips(gp.XOY_s(), 20, 12)).Edge()).Wire()
    # at (20, 0, 0) the ellipse runs along +y
    section = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(
        gp_Circ(gp_Ax2(gp_Pnt(20, 0, 0), gp_Dir(0, 1, 0)), radius)).Edge()).Wire()
    pipe = BRepOffsetAPI_MakePipeShell(path)
    pipe.SetMode(True)
    pipe.Add(section)
    pipe.Build()
    if not pipe.IsDone() or not pipe.MakeSolid():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the circle round the ellipse")
    return pipe.Shape()


def build():
    wall = BRepAlgoAPI_Cut(tube(3), tube(2))
    if not wall.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not hollow the tube")
    half = BRepPrimAPI_MakeBox(gp_Pnt(-30, -30, -10), gp_Pnt(15, 30, 10)).Shape()
    common = BRepAlgoAPI_Common(wall.Shape(), half)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common could not cut the wall open")
    return common.Shape()
