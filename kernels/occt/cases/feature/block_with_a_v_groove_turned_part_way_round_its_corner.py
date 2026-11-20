"""A block with a V-groove turned part way round its corner, ending blind: the block
0 ≤ x, y ≤ 12, 0 ≤ z ≤ 10; the triangle (3, 0, 10), (9, 0, 10), (6, 0, 5) in the plane y = 0 turned
about the z axis, towards +y, through 50°, and cut away."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Pnt

from bench import Refused

TRIANGLE = [(3, 10), (9, 10), (6, 5)]  # (x, z) in the plane y = 0
TURN = 50  # degrees


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(12, 12, 10)).Shape()
    outline = BRepBuilderAPI_MakePolygon()
    for x, z in TRIANGLE:
        outline.Add(gp_Pnt(x, 0, z))
    outline.Close()
    face = BRepBuilderAPI_MakeFace(outline.Wire()).Face()
    tool = BRepPrimAPI_MakeRevol(face, gp.OZ_s(), math.radians(TURN)).Shape()
    cut = BRepAlgoAPI_Cut(block, tool)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the groove")
    return cut.Shape()
