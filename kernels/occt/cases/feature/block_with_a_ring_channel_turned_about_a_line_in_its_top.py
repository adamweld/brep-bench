"""A block with a half-ring channel turned about a line in its top: the block 0 ≤ x ≤ 20,
−8 ≤ y ≤ 8, 0 ≤ z ≤ 8; the rectangle 5 ≤ x ≤ 15, 3 ≤ y ≤ 5 in the plane z = 8 turned a full turn
about the line y = 0, z = 8 (along x) and cut away."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.gp import gp_Ax1, gp_Dir, gp_Pnt

from bench import Refused


def build():
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, -8, 0), gp_Pnt(20, 8, 8)).Shape()
    outline = BRepBuilderAPI_MakePolygon()
    for x, y in [(5, 3), (15, 3), (15, 5), (5, 5)]:
        outline.Add(gp_Pnt(x, y, 8))
    outline.Close()
    face = BRepBuilderAPI_MakeFace(outline.Wire()).Face()
    tool = BRepPrimAPI_MakeRevol(face, gp_Ax1(gp_Pnt(0, 0, 8), gp_Dir(1, 0, 0))).Shape()
    cut = BRepAlgoAPI_Cut(block, tool)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the channel")
    return cut.Shape()
