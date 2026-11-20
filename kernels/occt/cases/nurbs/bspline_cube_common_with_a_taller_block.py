"""A B-spline cube and a block twice as high, their common part. The cube from (0, 0, 0) to
(10, 10, 10) has B-spline faces of degree 1; the block from (0, 0, 0) to (10, 10, 20) has planes."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 20)).Shape()
    common = BRepAlgoAPI_Common(cube, block)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and the block")
    return common.Shape()
