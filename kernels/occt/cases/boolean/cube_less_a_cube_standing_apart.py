"""A cube less an equal cube standing apart from it. A cube of side 10 from the origin; a second of
the same size, spanning x from 12 to 22 (a gap of 2) and y and z from 0 to 10. Cut the second from
the first."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 10)).Shape()
    tool = BRepPrimAPI_MakeBox(gp_Pnt(12, 0, 0), gp_Pnt(22, 10, 10)).Shape()
    cut = BRepAlgoAPI_Cut(cube, tool)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the two separate cubes")
    return cut.Shape()
