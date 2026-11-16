"""A rounded puck hollowed with walls thicker than its rounds. A puck: the cylinder of radius 6 on the z axis from z = 0 to z = 10 with both its circular edges rounded to
radius 2, each round a quarter of a torus whose tube has radius 2 about the circle of radius 4 at z = 2 or
z = 8. Hollow it to walls 3 thick: keep every point within 3 of its outside, leaving a closed cavity, every
point at least 3 inside it.

That is more than the rounds' radius. A round moved 3 inward would be a torus of tube radius 2 − 3 = −1,
turned inside out: it is no part of the cavity, whose side and ends run on past it to meet in sharp
edges. The cavity is the cylinder of radius 3 from z = 3 to z = 7 (the shrunk puck of
`offset/rounded-puck-shrunk-past-its-rounds`), and the result is the rounded puck less it."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeSolid
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.TopAbs import TopAbs_ShapeEnum
from OCP.TopoDS import TopoDS

from blend import edges_where, rounded
from edit import on_circle
from shell import hollow


def build():
    solid = BRepPrimAPI_MakeCylinder(6, 10).Shape()
    solid = rounded(solid, 2, edges_where(solid, on_circle((0, 0, 0), 6)) + edges_where(solid, on_circle((0, 0, 10), 6)), "the rims")
    result = hollow(solid, 3.0)
    # OCCT's offset may give the solid back as its closed shell; the solid is the one it bounds.
    if result.ShapeType() == TopAbs_ShapeEnum.TopAbs_SHELL:
        result = BRepBuilderAPI_MakeSolid(TopoDS.Shell(result)).Solid()
    return result
