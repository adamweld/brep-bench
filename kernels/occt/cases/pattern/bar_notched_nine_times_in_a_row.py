"""A bar notched nine times in a row, like a rack. A bar, the box from the origin to (100, 12, 20),
with a row of nine notches cut across its top, a linear pattern of one notch: like a rack's tooth
spaces.

The notch runs right across the bar, along y from y = 0 to y = 12. Its section in the plane y = 0 is
a trapezoid, symmetric about the line x = 10: 6 wide at the top face (z = 20), from x = 7 to x = 13,
narrowing to a flat floor 2 wide at z = 14, from x = 9 to x = 11. So it is 6 deep and its walls
slope 1 across for every 3 down.

The pattern repeats the notch along +x every 10: nine notches in all, centred on x = 10, 20, …, 90.
(A first notch at x = 0, on the bar's end, is not cut.) Neighbouring notches leave a flat land 4
wide on the top between them, and the bar's top is whole from x = 0 to 7 and from x = 93 to 100."""
from OCP.BRep import BRep_Builder
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_Transform)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.TopoDS import TopoDS_Compound
from OCP.gp import gp_Pnt, gp_Trsf, gp_Vec

from bench import Refused

COUNT, PITCH = 9, 10
RUN_OUT = 1  # the notch's tool runs on past the top face, on its slopes, and past both sides


def notch():
    """The first notch's tool, about x = 10: the trapezoid 2 wide at z = 14 whose walls slope
    1 across for 3 up, carried up past the top, extruded across the bar and past it."""
    top = 20 + RUN_OUT
    half_top = 1 + (top - 14) / 3
    outline = BRepBuilderAPI_MakePolygon()
    for x, z in [(10 - 1, 14), (10 + 1, 14), (10 + half_top, top), (10 - half_top, top)]:
        outline.Add(gp_Pnt(x, -RUN_OUT, z))
    outline.Close()
    face = BRepBuilderAPI_MakeFace(outline.Wire()).Face()
    return BRepPrimAPI_MakePrism(face, gp_Vec(0, 12 + 2 * RUN_OUT, 0)).Shape()


def build():
    bar = BRepPrimAPI_MakeBox(100, 12, 20).Shape()
    first = notch()
    # The linear pattern: the notch and its copies every PITCH along +x, cut in one go.
    tools = TopoDS_Compound()
    builder = BRep_Builder()
    builder.MakeCompound(tools)
    for k in range(COUNT):
        move = gp_Trsf()
        move.SetTranslation(gp_Vec(k * PITCH, 0, 0))
        builder.Add(tools, BRepBuilderAPI_Transform(first, move, True).Shape())
    cut = BRepAlgoAPI_Cut(bar, tools)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the row of notches")
    return cut.Shape()
