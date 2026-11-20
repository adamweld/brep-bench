"""A quarter torus from a disk turned a quarter turn. The disk of radius 2 centred on (10, 0, 0)
in the plane y = 0, turned 90 degrees about the z axis, from +x towards +y."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Ax2, gp_Circ, gp_Dir, gp_Pnt

CENTRE_X = 10
RADIUS = 2
ANGLE = 90  # degrees


def build():
    # the circle lies in the plane y = 0: its normal is the y axis
    circle = gp_Circ(gp_Ax2(gp_Pnt(CENTRE_X, 0, 0), gp_Dir(0, 1, 0)), RADIUS)
    wire = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(circle).Edge()).Wire()
    disk = BRepBuilderAPI_MakeFace(wire).Face()
    return BRepPrimAPI_MakeRevol(disk, gp.OZ_s(), math.radians(ANGLE)).Shape()
