"""A B-spline cube united with a block inside it, flush at three of its faces. A cube from (0, 0, 0)
to (10, 10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational
B-spline surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2
× 2 control points are the four corners of its face, so that it lies flat over the face. And a
block from (0, 0, 0) to (5, 5, 5), its faces planes: it fills one corner of the cube, three of its
faces lying in three of the cube's faces and its other three inside the cube. Unite them. The
block adds nothing, and the union is the cube."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    # converting the box makes each face's plane a B-spline of degree 1 each way over the face
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(5, 5, 5)).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, block)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the cube and the block")
    return fuse.Shape()
