"""Two cubes sharing a face, drilled through the face's centre. Two cubes of side 10 resting one on
the other, (−5, −5, 0) to (5, 5, 10) and (−5, −5, 10) to (5, 5, 20), kept as two solids, with a
hole of radius 3 drilled through both along the z axis."""
from OCP.BOPAlgo import BOPAlgo_Builder
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.collections import List_TopoDS_Shape
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused


def build():
    lower = BRepPrimAPI_MakeBox(gp_Pnt(-5, -5, 0), gp_Pnt(5, 5, 10)).Shape()
    upper = BRepPrimAPI_MakeBox(gp_Pnt(-5, -5, 10), gp_Pnt(5, 5, 20)).Shape()
    # A general fuse keeps both solids, sharing the face between them.
    glued = BOPAlgo_Builder()
    glued.AddArgument(lower)
    glued.AddArgument(upper)
    glued.Perform()
    if glued.HasErrors():
        raise Refused("BOPAlgo_Builder could not glue the cubes")
    bore = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -5), gp_Dir(0, 0, 1)), 3, 30).Shape()
    tools = List_TopoDS_Shape()
    tools.Append(bore)
    objects = List_TopoDS_Shape()
    objects.Append(glued.Shape())
    cut = BRepAlgoAPI_Cut()
    cut.SetArguments(objects)
    cut.SetTools(tools)
    cut.Build()
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not drill the cubes")
    return cut.Shape()
