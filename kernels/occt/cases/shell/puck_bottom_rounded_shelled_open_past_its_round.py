"""A puck with its bottom rim rounded, shelled open at its top past the round's radius. A puck: the cylinder of radius 6 on the z axis from z = 0 to z = 10 with its bottom edge rounded to radius 2,
the round a quarter of a torus whose tube has radius 2 about the circle of radius 4 at z = 2 (the top edge
is left sharp). Shell it inward to walls 3 thick, open at its top: every point of it within 3 of its faces
other than the top.

That is more than the round's radius. The round moved 3 inward would be a torus of tube radius 2 − 3 = −1,
turned inside out: it is no part of the cavity, whose wall and floor run on past it to meet in a sharp
edge. The cavity is the cylinder of radius 3 from z = 3 up through the top, and the result is the puck less
it, open at its top in the circle of radius 3."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeSolid
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.TopAbs import TopAbs_ShapeEnum
from OCP.TopoDS import TopoDS

from blend import edges_where, rounded
from edit import on_circle
from bench import face_through
from shell import shell


def build():
    solid = BRepPrimAPI_MakeCylinder(6, 10).Shape()
    solid = rounded(solid, 2, edges_where(solid, on_circle((0, 0, 0), 6)), "the rims")
    result = shell(solid, [face_through(solid, (0, 0, 10))], 3.0)
    # OCCT's offset may give the solid back as its closed shell; the solid is the one it bounds.
    if result.ShapeType() == TopAbs_ShapeEnum.TopAbs_SHELL:
        result = BRepBuilderAPI_MakeSolid(TopoDS.Shell(result)).Solid()
    return result
