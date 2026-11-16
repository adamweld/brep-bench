"""A hemisphere with its rim rounded, hollowed with walls thicker than the round. The half of the ball of radius 5 about the origin above the plane z = 0, with its rim, where the flat meets the ball,
rounded to radius 1: the round is a piece of a torus whose tube has radius 1 about the circle of radius √15 at z = 1,
tangent to the flat along the circle of radius √15 and to the ball along the circle through (5√15/4, 5/4). Hollow it to walls 2 thick: keep every point within 2 of its outside, leaving a closed cavity, every point at
least 2 inside it.

That is more than the round's radius, so the round moved inward turns inside out and drops, and the cavity is the
hemisphere shrunk by 2 with a sharp edge: the part of the ball of radius 3 about the origin above the plane z = 2. The
result is the rounded hemisphere less that cavity."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeSolid
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeSphere
from OCP.TopAbs import TopAbs_ShapeEnum
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from blend import edges_where, rounded
from edit import on_circle
from shell import hollow


def build():
    above = BRepPrimAPI_MakeBox(gp_Pnt(-6, -6, 0), gp_Pnt(6, 6, 6)).Shape()
    half = BRepAlgoAPI_Common(BRepPrimAPI_MakeSphere(5).Shape(), above).Shape()
    solid = rounded(half, 1, edges_where(half, on_circle((0, 0, 0), 5)), "the rim")
    result = hollow(solid, 2.0)
    # OCCT's offset may give the solid back as its closed shell; the solid is the one it bounds.
    if result.ShapeType() == TopAbs_ShapeEnum.TopAbs_SHELL:
        result = BRepBuilderAPI_MakeSolid(TopoDS.Shell(result)).Solid()
    return result
