"""A crescent turned three quarters of a turn about the line through its tips. In the xz plane, the
disc of radius 5 centered at (0, 0) less the disc of radius 5 centered at (6, 0), turned about the
line x = 3, z = 0 along y by 270°, right-handed about +y."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.Geom import Geom_Circle
from OCP.gp import gp_Ax1, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def disc(cx):
    # a disc in the xz plane: the circle's normal is y
    circle = Geom_Circle(gp_Ax2(gp_Pnt(cx, 0, 0), gp_Dir(0, 1, 0), gp_Dir(1, 0, 0)), 5)
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(circle).Edge()).Wire()
    return BRepBuilderAPI_MakeFace(wire).Face()


def build():
    crescent = BRepAlgoAPI_Cut(disc(0), disc(6))
    if not crescent.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the second disc from the first")
    axis = gp_Ax1(gp_Pnt(3, 0, 0), gp_Dir(0, 1, 0))
    revolved = BRepPrimAPI_MakeRevol(crescent.Shape(), axis, math.radians(270))
    if not revolved.IsDone():
        raise Refused("BRepPrimAPI_MakeRevol could not turn the crescent")
    return revolved.Shape()
