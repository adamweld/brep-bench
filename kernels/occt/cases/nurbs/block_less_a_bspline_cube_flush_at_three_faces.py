"""A block less a B-spline cube, the block flush at three faces and overhanging. A block from (0, 0,
0) to (10, 4, 20), its faces planes. And a cube from (0, 0, 0) to (10, 10, 10) whose six faces are
B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in each
direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four
corners of its face, so that it lies flat over the face. Cut the cube from the block. The block's
two ends and its sides at y = 0 and z = 0 lie in the cube's faces x = 0, x = 10, y = 0 and z = 0,
and it runs on above the cube's top; its other long side, at y = 4, passes through the cube. What
is left is the part above the cube, the box 10 by 4 by 10 from (0, 0, 10) to (10, 4, 20)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    # converting the box makes each face's plane a B-spline of degree 1 each way over the face
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 4, 20)).Shape()
    cut = BRepAlgoAPI_Cut(block, cube)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cube and the block")
    return cut.Shape()
