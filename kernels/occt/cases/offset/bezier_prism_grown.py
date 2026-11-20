"""A prism with a Bezier wall grown by 1. The straight extrusion, 5 along +z from the plane z = 0, of
the region of the xy plane bounded by the line from (0, 0) to (10, 0), the cubic Bezier curve (a
cubic B-spline of one span) with control points (10, 0), (10, 5), (6, 8) and (0, 8), and the line
from (0, 8) back to (0, 0). The region is convex. So the solid has a curved side wall, the
extrusion of the Bezier curve, and flat faces: the bottom, the top and two walls, in the planes y
= 0 and x = 0."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused
from offset import offset
from turned import bezier

CURVE = [(10, 0, 0), (10, 5, 0), (6, 8, 0), (0, 8, 0)]
THICKNESS = 5
GROWTH = 1


def build():
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 0), gp_Pnt(10, 0, 0)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(bezier(CURVE)).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(0, 8, 0), gp_Pnt(0, 0, 0)).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire(), True)
    if not face.IsDone():
        raise Refused("BRepBuilderAPI_MakeFace made no face of the profile")
    prism = BRepPrimAPI_MakePrism(face.Face(), gp_Vec(0, 0, THICKNESS)).Shape()
    return offset(prism, GROWTH)
