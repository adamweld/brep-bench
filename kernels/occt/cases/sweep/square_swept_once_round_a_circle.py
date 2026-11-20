"""A square swept once round a circle. The square with corners (4, 0, 0), (6, 0, 0), (6, 0, 2) and
(4, 0, 2), in the plane y = 0, swept once round the circle of radius 4 about the z axis in the xy
plane, which starts at the square's corner (4, 0, 0)."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakePolygon, BRepBuilderAPI_MakeWire)
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.gp import gp, gp_Circ, gp_Pnt

from bench import Refused


def build():
    path = BRepBuilderAPI_MakeWire(
        BRepBuilderAPI_MakeEdge(gp_Circ(gp.XOY_s(), 4), 0, 2 * math.pi).Edge()).Wire()
    square = BRepBuilderAPI_MakePolygon(gp_Pnt(4, 0, 0), gp_Pnt(6, 0, 0), gp_Pnt(6, 0, 2),
                                        gp_Pnt(4, 0, 2), True).Wire()
    pipe = BRepOffsetAPI_MakePipeShell(path)
    pipe.SetMode(True)  # Frenet: the square stays in the plane through the z axis
    pipe.Add(square)
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the square")
    if not pipe.MakeSolid():
        raise Refused("the swept square did not close into a solid")
    return pipe.Shape()
