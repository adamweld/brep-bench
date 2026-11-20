"""A B-spline cube less a block touching it at a corner. A cube from (0, 0, 0) to (10, 10, 10) whose
six faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1
in each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the
four corners of its face, so that it lies flat over the face. And a block from (10, 10, 10) to
(20, 20, 20), its faces planes. Cut the block from the cube. The block touches the cube at the
single point (10, 10, 10), the cube's far corner, and nowhere else. Nothing is removed: the result
is the whole cube."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    # converting the box makes each face's plane a B-spline of degree 1 each way over the face
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(10, 10, 10), gp_Pnt(20, 20, 20)).Shape()
    cut = BRepAlgoAPI_Cut(cube, block)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cube and the block")
    return cut.Shape()
