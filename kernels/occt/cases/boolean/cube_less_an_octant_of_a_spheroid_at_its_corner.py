"""A cube less a prolate spheroid centred on its corner. The cube from the origin to (10, 10, 10),
less the ellipsoid centred on the origin, axis of revolution z, semi-axes 3, 3 and 5."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.Geom import Geom_Ellipse
from OCP.gp import gp_Ax1, gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    # The half ellipse in the xz plane (x >= 0), major radius 5 along z, minor 3 along x, closed by
    # the axis, turned about the z axis.
    frame = gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 1, 0), gp_Dir(0, 0, 1))
    ellipse = Geom_Ellipse(frame, 5, 3)
    arc = BRepBuilderAPI_MakeEdge(ellipse, 0, math.pi).Edge()
    wire = BRepBuilderAPI_MakeWire(arc)
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 5), gp_Pnt(0, 0, -5)).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    spheroid = BRepPrimAPI_MakeRevol(face, gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1))).Shape()
    cut = BRepAlgoAPI_Cut(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), spheroid)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut failed")
    return cut.Shape()
