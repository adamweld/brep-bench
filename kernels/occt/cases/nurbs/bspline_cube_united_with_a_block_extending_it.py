"""A B-spline cube united with a block that carries it twice as high. A cube from (0, 0, 0) to (10,
10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline
surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2
control points are the four corners of its face, so that it lies flat over the face. And a block
from (0, 0, 0) to (10, 10, 20), its faces planes. Unite them. The block's four sides lie in the
cube's four sides and its bottom in the cube's bottom, and it is taller than the cube: the union
is the block, a box 10 by 10 by 20, its four sides each one flat face where the two operands'
faces met."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    # converting the box makes each face's plane a B-spline of degree 1 each way over the face
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 20)).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, block)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the cube and the block")
    return fuse.Shape()
