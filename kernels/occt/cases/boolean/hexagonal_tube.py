"""A hexagonal tube. A regular hexagonal prism, corners at distance 5, 8 high, with the coaxial prism
of corners at distance 3 cut away: open at both ends."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

HEIGHT = 8


def prism(radius, z0, height):
    poly = BRepBuilderAPI_MakePolygon()
    for k in range(6):
        a = math.radians(60 * k)
        poly.Add(gp_Pnt(radius * math.cos(a), radius * math.sin(a), z0))
    poly.Close()
    face = BRepBuilderAPI_MakeFace(poly.Wire(), True).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, height)).Shape()


def build():
    # the inner prism runs past both ends
    cut = BRepAlgoAPI_Cut(prism(5, 0, HEIGHT), prism(3, -1, HEIGHT + 2))
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two prisms")
    return cut.Shape()
