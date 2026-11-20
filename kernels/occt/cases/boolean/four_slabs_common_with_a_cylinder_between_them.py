"""Four slabs in a grid, their common part with a cylinder standing at their junction. Four slabs
10 x 10 x 2 (z from 0 to 2, squares [1, 11] or [-11, -1] in x and y) common with a cylinder of
radius 3 on the z axis from z = -3 to z = 5."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.collections import List_TopoDS_Shape
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    slabs = List_TopoDS_Shape()
    for x in (-11, 1):
        for y in (-11, 1):
            slabs.Append(BRepPrimAPI_MakeBox(gp_Pnt(x, y, 0), gp_Pnt(x + 10, y + 10, 2)).Shape())
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -3), gp.DZ_s()), 3, 8).Shape()
    tools = List_TopoDS_Shape()
    tools.Append(cylinder)
    common = BRepAlgoAPI_Common()
    common.SetArguments(slabs)
    common.SetTools(tools)
    common.Build()
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the slabs and the cylinder")
    return common.Shape()
