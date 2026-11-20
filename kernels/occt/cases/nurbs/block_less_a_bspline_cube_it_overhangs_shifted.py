"""A block less a B-spline cube it overhangs, shifted up. A block from (0, 0, 5) to (10, 10, 15), its
faces planes. And a cube from (0, 0, 0) to (10, 10, 10) whose six faces are B-spline surfaces, not
planes: each is the non-rational B-spline surface of degree 1 in each direction, with knot vectors
(0, 0, 1, 1) both ways, whose 2 × 2 control points are the four corners of its face, so that it
lies flat over the face. Cut the cube from the block. The block's four sides lie in the cube's
four sides; its bottom is inside the cube, halfway up, and its top is above the cube's top. What
is left is the part of the block above the cube, from (0, 0, 10) to (10, 10, 15)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    # converting the box makes each face's plane a B-spline of degree 1 each way over the face
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 5), gp_Pnt(10, 10, 15)).Shape()
    cut = BRepAlgoAPI_Cut(block, cube)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cube and the block")
    return cut.Shape()
