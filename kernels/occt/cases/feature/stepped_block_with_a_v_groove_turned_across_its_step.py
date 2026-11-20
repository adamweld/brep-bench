"""A stepped block with a V-groove turned across its step: the lower box 0 ≤ x, y ≤ 20, 0 ≤ z ≤ 6
and the upper box 0 ≤ x, y ≤ 10, 6 ≤ z ≤ 12, fused; the triangle (6, 0, 12), (14, 0, 12), (10, 0, 2)
in the plane y = 0 turned about the z axis, towards +y, a quarter turn to the plane x = 0, and cut away."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeRevol
from OCP.gp import gp, gp_Pnt

from bench import Refused

TRIANGLE = [(6, 12), (14, 12), (10, 2)]  # (x, z) in the plane y = 0


def build():
    lower = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(20, 20, 6)).Shape()
    upper = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 6), gp_Pnt(10, 10, 12)).Shape()
    fuse = BRepAlgoAPI_Fuse(lower, upper)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the stepped block")
    outline = BRepBuilderAPI_MakePolygon()
    for x, z in TRIANGLE:
        outline.Add(gp_Pnt(x, 0, z))
    outline.Close()
    face = BRepBuilderAPI_MakeFace(outline.Wire()).Face()
    tool = BRepPrimAPI_MakeRevol(face, gp.OZ_s(), math.radians(90)).Shape()
    cut = BRepAlgoAPI_Cut(fuse.Shape(), tool)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the groove")
    return cut.Shape()
