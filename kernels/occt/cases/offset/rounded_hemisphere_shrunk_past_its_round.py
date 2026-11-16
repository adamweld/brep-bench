"""A hemisphere with its rim rounded, shrunk past the round's radius. The half of the ball of radius 5 about the origin above the plane z = 0, with its rim, where the flat meets the ball,
rounded to radius 1: the round is a piece of a torus whose tube has radius 1 about the circle of radius √15 at z = 1,
tangent to the flat along the circle of radius √15 and to the ball along the circle through (5√15/4, 5/4). Shrink it by 3/2: the solid of every point at least 3/2 from its outside.

That is more than the round's radius. The round moved 3/2 inward would be a torus of tube radius 1 − 3/2 < 0, turned
inside out, and drops; the moved flat and the moved ball meet in a sharp edge. The rounded hemisphere lies between the
hemisphere shrunk by 1 and grown back by 1 and the hemisphere itself, and both shrink by 3/2 to the same solid: the
hemisphere's points at least 3/2 inside, the part of the ball of radius 7/2 about the origin above the plane z = 3/2."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeSolid
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.TopAbs import TopAbs_ShapeEnum
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from blend import edges_where, rounded
from edit import on_circle
from offset import offset


def build():
    above = BRepPrimAPI_MakeBox(gp_Pnt(-6, -6, 0), gp_Pnt(6, 6, 6)).Shape()
    half = BRepAlgoAPI_Common(BRepPrimAPI_MakeSphere(5).Shape(), above).Shape()
    solid = rounded(half, 1, edges_where(half, on_circle((0, 0, 0), 5)), "the rim")
    result = offset(solid, -1.5)
    # OCCT's offset may give the solid back as its closed shell; the solid is the one it bounds.
    if result.ShapeType() == TopAbs_ShapeEnum.TopAbs_SHELL:
        result = BRepBuilderAPI_MakeSolid(TopoDS.Shell(result)).Solid()
    return result
