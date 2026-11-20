"""A B-spline cube and a block sharing three of its face planes, their common part. The cube from
(0, 0, 0) to (10, 10, 10) with every face a B-spline surface of degree 1 each way whose four
control points are the face's corners, and a block from (0, 0, 0) to (15, 7, 8), its faces planes."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from bench import Refused


def build():
    # converting the box makes each face's plane a B-spline of degree 1 each way over the face
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    block = BRepPrimAPI_MakeBox(15, 7, 8).Shape()
    common = BRepAlgoAPI_Common(cube, block)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and the block")
    return common.Shape()
