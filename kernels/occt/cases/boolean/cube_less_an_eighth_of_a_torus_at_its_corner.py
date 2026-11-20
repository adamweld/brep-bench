"""A cube less a torus centred on its corner. The cube from the origin to (10, 10, 10), less the torus about the z axis centred on the origin: tube radius 2, its centre running round the circle of radius 5 in the plane z = 0."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeTorus
from OCP.gp import gp

from bench import Refused


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    tool = BRepPrimAPI_MakeTorus(gp.XOY_s(), 5, 2).Shape()
    cut = BRepAlgoAPI_Cut(cube, tool)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut failed")
    return cut.Shape()
