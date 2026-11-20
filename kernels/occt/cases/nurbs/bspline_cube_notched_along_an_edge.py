"""A B-spline cube notched along an edge by a block. A cube from (0, 0, 0) to (10, 10, 10) whose six
faces are B-spline surfaces, not planes: each is the non-rational B-spline surface of degree 1 in
each direction, with knot vectors (0, 0, 1, 1) both ways, whose 2 × 2 control points are the four
corners of its face, so that it lies flat over the face. And a block from (0, 0, 0) to (10, 4, 4),
its faces planes. Cut the block from the cube. The block's long side lies along the cube's bottom
edge at y = 0, z = 0, from end to end, so its two ends lie in the cube's end faces and two of its
sides in the cube's faces y = 0 and z = 0. A square notch of 4 by 4 is left along the edge."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_NurbsConvert
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from bench import Refused


def build():
    # converting the box makes each face's plane a B-spline of degree 1 each way over the face
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    block = BRepPrimAPI_MakeBox(gp_Pnt(0, 0, 0), gp_Pnt(10, 4, 4)).Shape()
    cut = BRepAlgoAPI_Cut(cube, block)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the cube and the block")
    return cut.Shape()
