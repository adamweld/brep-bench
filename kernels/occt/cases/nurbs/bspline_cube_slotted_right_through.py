"""A B-spline cube slotted right through by a block. A cube from (0, 0, 0) to (10, 10, 10) whose six
faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in
each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four
corners of its face, so that it lies flat over the face. And a block from (0, 0, 3) to (10, 10,
7), its faces planes. Cut the block from the cube. The block's four sides lie in the cube's four
sides, and its bottom and top are inside the cube, at heights 3 and 7. It cuts a slot through the
whole width of the cube, and what is left is two separate slabs 3 thick: the box from (0, 0, 0) to
(10, 10, 3) and the box from (0, 0, 7) to (10, 10, 10)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    # converting the box makes each face's plane a B-spline of degree 1 each way over the face
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 3), gp_Pnt(10, 10, 7)).Shape()
    cut = BRepAlgoAPI_Cut(cube, block)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cube and the block")
    return cut.Shape()
