"""A U-shaped bar notched across its arms, the notch flush with the slot end. The bar from z = 0 to z = 6 on the U with corners
(0, 0), (20, 0), (20, 4), (8, 4), (8, 8), (20, 8), (20, 12), (0, 12), less the box from (8, -2, 2) to (14, 14, 10)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused


def prism(points, z0, z1):
    """A prism on the polygon `points` (x, y), from z = z0 to z = z1."""
    wire = BRepBuilderAPI_MakePolygon()
    for x, y in points:
        wire.Add(gp_Pnt(x, y, z0))
    wire.Close()
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(0, 0, z1 - z0)).Shape()


def build():
    bar = prism([(0, 0), (20, 0), (20, 4), (8, 4), (8, 8), (20, 8), (20, 12), (0, 12)], 0, 6)
    box = BRepPrimAPI_MakeBox(gp_Pnt(8, -2, 2), gp_Pnt(14, 14, 10)).Shape()
    cut = BRepAlgoAPI_Cut(bar, box)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the bar and the box")
    return cut.Shape()
