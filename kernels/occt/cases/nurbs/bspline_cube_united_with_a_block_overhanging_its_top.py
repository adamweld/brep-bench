"""A B-spline cube united with a block resting on its top and overhanging an edge. The cube from
(0, 0, 0) to (10, 10, 10) has B-spline faces of degree 1; the block from (0, -5, 10) to
(10, 5, 15) has planes."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    # converting the box makes each face's plane a B-spline of degree 1 each way over the face
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, -5, 10), gp_Pnt(10, 5, 15)).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, block)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the cube and the block")
    return fuse.Shape()
