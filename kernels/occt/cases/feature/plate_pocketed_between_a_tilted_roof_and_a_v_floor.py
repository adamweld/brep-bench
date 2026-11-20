"""A plate pocketed between a tilted roof and a V-shaped floor: the plate 0 ≤ x ≤ 40, 0 ≤ y ≤ 30,
0 ≤ z ≤ 6, and over 15 ≤ x ≤ 25, 10 ≤ y ≤ 20 the part between the roof z = 8 − 0.3(x − 15) and the
floor z = 1.5 + 0.2|x − 20| cut away. The tool is that region's profile in the xz plane, extruded
along y."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused

PROFILE = [(15, 2.5), (20, 1.5), (25, 2.5), (25, 5), (15, 8)]  # (x, z)


def build():
    plate = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(40, 30, 6)).Shape()
    wire = BRepBuilderAPI_MakePolygon()
    for x, z in PROFILE:
        wire.Add(gp_Pnt(x, 10, z))
    wire.Close()
    tool = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(wire.Wire()).Face(), gp_Vec(0, 10, 0)).Shape()
    cut = BRepAlgoAPI_Cut(plate, tool)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the pocket")
    return cut.Shape()
