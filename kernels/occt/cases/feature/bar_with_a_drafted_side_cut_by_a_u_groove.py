"""A bar with a drafted side and rounded top edges, cut by a U-shaped groove. The bar is 20 wide
(x), 30 long (y) and 10 high, its x = 20 side drafted 10° (leaning in as it rises) and its two long
top edges rounded to 2. The groove is cut from the top to the floor z = 7, 3 wide, along the open U
(30, 7.5) – (5.5, 7.5) – (5.5, 22.5) – (30, 22.5), whose open ends are outside the bar."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from drafted_bar import FLOOR, bar
from bench import Refused

# the U's strip, its centre line offset 1.5 to each side, from the floor up past the top
GROOVE = [(30, 6), (4, 6), (4, 24), (30, 24), (30, 21), (7, 21), (7, 9), (30, 9)]


def build():
    wire = BRepBuilderAPI_MakePolygon()
    for x, y in GROOVE:
        wire.Add(gp_Pnt(x, y, FLOOR))
    wire.Close()
    tool = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(wire.Wire()).Face(), gp_Vec(0, 0, 5)).Shape()
    cut = BRepAlgoAPI_Cut(bar(), tool)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the groove")
    return cut.Shape()
