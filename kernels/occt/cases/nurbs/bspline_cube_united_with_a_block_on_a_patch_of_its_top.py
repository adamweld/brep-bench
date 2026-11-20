"""A B-spline cube united with a block standing on a patch of its top. A cube from (0, 0, 0) to (10,
10, 10) whose six faces are B-spline surfaces, not planes: each is the non-rational B-spline
surface of degree 1 in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2
control points are the four corners of its face, so that it lies flat over the face. And a block
from (0, 0, 10) to (5, 5, 15), its faces planes. Unite them. The block stands on the cube's top
face at one corner, its bottom a 5 by 5 patch of that face and its two outer sides in the planes
of the cube's x = 0 and y = 0 faces; no volume overlaps. The union is the cube with a 5 by 5 by 5
step on one corner of its top."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    # converting the box makes each face's plane a B-spline of degree 1 each way over the face
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 10), gp_Pnt(5, 5, 15)).Shape()
    fuse = BRepAlgoAPI_Fuse(cube, block)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the cube and the block")
    return fuse.Shape()
