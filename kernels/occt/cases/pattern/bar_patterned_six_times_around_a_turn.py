"""A bar patterned six times around a turn, each copy lying on another. A bar patterned about the z
axis: six copies, every 60° around a whole turn, joined into one solid.

The bar is the box −20 ≤ x ≤ 20, −3 ≤ y ≤ 3, 0 ≤ z ≤ 5. The copies are the bar itself and the bar
turned about the z axis by 60°, 120°, 180°, 240° and 300°.

The bar is symmetric under a half turn about the z axis, so each copy lies exactly on the one three
steps from it: the bar turned 180° is the bar again. The six copies make three distinct bars
crossing at the axis, a star of six arms, every point of it covered by two copies.

The star's section: six arms 6 wide, meeting in a regular hexagon of inradius 3 about the axis. The
facing sides of two neighbouring arms meet at an inner corner 6 from the axis, on the line midway
between them. So each side of an arm runs 20 − 3√3 from that corner out to the arm's end, which is
flat, 20 from the axis."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_Transform
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.collections import List_TopoDS_Shape
from OCP.gp import gp, gp_Pnt, gp_Trsf

from bench import Refused

COUNT = 6


def build():
    bar = BRepPrimAPI_MakeBox(gp_Pnt(-20, -3, 0), 40, 6, 5).Shape()
    copies = List_TopoDS_Shape()
    for k in range(1, COUNT):
        turn = gp_Trsf()
        turn.SetRotation(gp.OZ_s(), math.radians(k * 360 / COUNT))
        copies.Append(BRepBuilderAPI_Transform(bar, turn, True).Shape())
    first = List_TopoDS_Shape()
    first.Append(bar)
    # One fuse of the bar with all five turned copies, each lying exactly on the one opposite it.
    star = BRepAlgoAPI_Fuse()
    star.SetArguments(first)
    star.SetTools(copies)
    star.Build()
    if not star.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the six copies")
    star.SimplifyResult()
    return star.Shape()
