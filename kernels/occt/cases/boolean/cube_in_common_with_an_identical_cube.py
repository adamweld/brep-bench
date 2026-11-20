"""Two cubes of side 10 on the same place, their common part."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from bench import Refused


def build():
    first = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    second = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    result = BRepAlgoAPI_Common(first, second)
    if not result.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the two")
    return result.Shape()
