"""A cylinder less four slabs in a grid around it. A cylinder of radius 3 on the z axis from
z = -3 to z = 5, less four slabs 10 x 10 x 2 (z from 0 to 2, squares [1, 11] or [-11, -1] in x
and y)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.collections import List_TopoDS_Shape
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused


def build():
    cylinder = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -3), gp.DZ_s()), 3, 8).Shape()
    slabs = List_TopoDS_Shape()
    for x in (-11, 1):
        for y in (-11, 1):
            slabs.Append(BRepPrimAPI_MakeBox(gp_Pnt(x, y, 0), gp_Pnt(x + 10, y + 10, 2)).Shape())
    arguments = List_TopoDS_Shape()
    arguments.Append(cylinder)
    cut = BRepAlgoAPI_Cut()
    cut.SetArguments(arguments)
    cut.SetTools(slabs)
    cut.Build()
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cylinder and the slabs")
    return cut.Shape()
