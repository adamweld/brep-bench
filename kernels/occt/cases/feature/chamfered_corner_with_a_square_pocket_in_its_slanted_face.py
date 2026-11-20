"""A cube with an edge chamfered and a square pocket cut square into the chamfer. A cube of side 10
between the origin and (10, 10, 10), the edge where its faces x = 10 and z = 10 meet chamfered by 3
on each face (the plane x + z = 17). A square pocket 2 by 2, 1.5 deep, cut square into the chamfer
face, centred on its centre (8.5, 5, 8.5), its floor parallel to the chamfer."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

S2 = math.sqrt(2)
DEPTH = 1.5


def prism(points, along):
    polygon = BRepBuilderAPI_MakePolygon()
    for p in points:
        polygon.Add(gp_Pnt(*p))
    polygon.Close()
    return BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(polygon.Wire()).Face(), gp_Vec(*along)).Shape()


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    # The chamfer: the triangular prism above the plane x + z = 17, meeting the cube's faces exactly.
    wedge = prism([(7, 0, 10), (10, 0, 10), (10, 0, 7)], (0, 10, 0))
    chamfered = BRepAlgoAPI_Cut(cube, wedge)
    if not chamfered.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the chamfer")
    # The pocket: a square on the chamfer, 2 along y and 2 up the slope (1, 0, -1)/sqrt 2 about
    # (8.5, 5, 8.5), taken DEPTH along the inward normal (-1, 0, -1)/sqrt 2.
    corners = [(8.5 + a / S2, 5 + b, 8.5 - a / S2) for a, b in [(-1, -1), (1, -1), (1, 1), (-1, 1)]]
    pocket = prism(corners, (-DEPTH / S2, 0, -DEPTH / S2))
    cut = BRepAlgoAPI_Cut(chamfered.Shape(), pocket)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the pocket")
    return cut.Shape()
