"""A plate with two slots, extruded from its outline. The outline in the plane z = 0 is the
rectangle [0, 60] x [0, 40] with two stadium-shaped holes (end centres (15, 12), (45, 12) and
(15, 28), (45, 28), radius 4); it is extruded 2 up and 4 down."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.TopLoc import TopLoc_Location
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused


def segment(a, b):
    return BRepBuilderAPI_MakeEdge(gp_Pnt(*a, 0), gp_Pnt(*b, 0)).Edge()


def arc(centre, radius, start, end):
    circle = gp_Circ(gp_Ax2(gp_Pnt(*centre, 0), gp_Dir(0, 0, 1)), radius)
    return BRepBuilderAPI_MakeEdge(circle, start, end).Edge()


def slot(x0, x1, y, r):
    wire = BRepBuilderAPI_MakeWire()
    for e in (segment((x0, y - r), (x1, y - r)), arc((x1, y), r, -math.pi / 2, math.pi / 2),
              segment((x1, y + r), (x0, y + r)), arc((x0, y), r, math.pi / 2, 3 * math.pi / 2)):
        wire.Add(e)
    return wire.Wire()


def build():
    outer = BRepBuilderAPI_MakeWire()
    corners = [(0, 0), (60, 0), (60, 40), (0, 40)]
    for i in range(4):
        outer.Add(segment(corners[i], corners[(i + 1) % 4]))
    face = BRepBuilderAPI_MakeFace(outer.Wire(), True)
    for y in (12, 28):
        hole = slot(15, 45, y, 4)
        hole.Reverse()
        face.Add(hole)
    if not face.IsDone():
        raise Refused("BRepBuilderAPI_MakeFace declined the outline")
    # 2 up and 4 down from the plane z = 0: the prism from z = -4 to z = 2
    return BRepPrimAPI_MakePrism(face.Face().Moved(TopLoc_Location(gp_Trsf_translation(0, 0, -4))),
                                 gp_Vec(0, 0, 6)).Shape()


def gp_Trsf_translation(x, y, z):
    t = gp_Trsf()
    t.SetTranslation(gp_Vec(x, y, z))
    return t
