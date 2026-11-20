"""A B-spline cube united with a block standing on its whole top. A cube from (0, 0, 0) to (10, 10,
10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface
of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control
points are the four corners of its face, so that it lies flat over the face. And a block from (0,
0, 10) to (10, 10, 15), its faces planes. Unite them. The block's bottom face is the cube's top
face and no volume overlaps: the union is a box 10 by 10 by 15, its sides each one flat face where
the two operands' faces met."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    # converting the box makes each face's plane a B-spline of degree 1 each way over the face
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 10), gp_Pnt(10, 10, 15)).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, block)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the cube and the block")
    return fuse.Shape()
