"""A B-spline cube and a diamond prism, their common part. The cube from (0, 0, 0) to (10, 10, 10)
with every face a B-spline surface of degree 1 each way whose four control points are the face's
corners, and the square prism from z = 0 to z = 10 on the square with corners (0, 0), (10, 10),
(20, 0) and (10, −10): one of its sides runs along the diagonal of the cube's bottom face, its
corners (0, 0) and (10, 10) are corners of the cube, and its diagonal from (0, 0) to (20, 0) runs
along an edge of the cube."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_NurbsConvert)
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakePrism
from OCP.gp import gp_Pnt, gp_Vec

from bench import Refused


def build():
    # converting the box makes each face's plane a B-spline of degree 1 each way over the face
    cube = BRepBuilderAPI_NurbsConvert(BRepPrimAPI_MakeBox(10, 10, 10).Shape(), True).Shape()
    square = BRepBuilderAPI_MakePolygon(gp_Pnt(0, 0, 0), gp_Pnt(10, 10, 0), gp_Pnt(20, 0, 0),
                                        gp_Pnt(10, -10, 0), True).Wire()
    diamond = BRepPrimAPI_MakePrism(BRepBuilderAPI_MakeFace(square).Face(),
                                    gp_Vec(0, 0, 10)).Shape()
    common = BRepAlgoAPI_Common(cube, diamond)
    if not common.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the cube and the diamond")
    return common.Shape()
