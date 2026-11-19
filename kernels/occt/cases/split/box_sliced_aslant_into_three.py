"""A box sliced aslant into three solids. A box from the origin to (30, 20, 10), split by the two
parallel planes x + y + z = 15 and x + y + z = 35, square to the direction (1, 1, 1), into three
solids, kept as three. They are the piece where x + y + z ≤ 15, the slab between the planes, and the
piece where x + y + z ≥ 35. They touch along the cuts but are not joined: each cut is a face of both
the pieces beside it.

The first cut is the quadrilateral (5, 0, 10), (15, 0, 0), (0, 15, 0), (0, 5, 10), and the second
the pentagon (25, 0, 10), (30, 0, 5), (30, 5, 0), (15, 20, 0), (5, 20, 10). The pieces' volumes are
1625/3, 6875/2 and 12125/6."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Splitter
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.collections import List_TopoDS_Shape
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt

from bench import Refused

SIZE = (30, 20, 10)
CUTS = (15, 35)  # x + y + z = c


def build():
    box = BRepPrimAPI_MakeBox(*SIZE).Shape()
    arguments, tools = List_TopoDS_Shape(), List_TopoDS_Shape()
    arguments.Append(box)
    for c in CUTS:
        plane = gp_Pln(gp_Pnt(c / 3, c / 3, c / 3), gp_Dir(1, 1, 1))
        tools.Append(BRepBuilderAPI_MakeFace(plane, -60, 60, -60, 60).Face())  # past the box
    splitter = BRepAlgoAPI_Splitter()
    splitter.SetArguments(arguments)
    splitter.SetTools(tools)
    splitter.Build()
    if not splitter.IsDone():
        raise Refused("BRepAlgoAPI_Splitter could not slice the box")
    return splitter.Shape()  # a compound of the three pieces
