"""A rounded cone frustum hollowed with walls thicker than its rounds. A cone frustum on the z axis, from the disk of radius 6 at z = 0 to the disk of radius 3 at z = 6 (its side
the cone 2r + z = 12), with both rims rounded to radius 1.5, each round a piece of a torus tangent to the flat face and to
the cone. Hollow it to walls 2 thick: keep every point within 2 of its outside, leaving a closed cavity, every point at least 2 inside: the frustum from z = 2 to z = 4, its side the cone 2r + z = 12 − 2√5. The walls are thicker than the rounds' radius, so the rounds moved inward turn inside out and drop, and the cavity is the sharp frustum shrunk by the wall, as in `offset/rounded-frustum-shrunk-past-its-rounds`.

The result is the rounded frustum less that cavity. The rounded frustum's measures follow from Pappus's theorem on its
half-section, the trapezoid (0, 0), (6, 0), (3, 6), (0, 6) with the rounded corner(s) cut to arcs of radius 1.5: each arc's
corner region is the kite between the corner, the arc's two tangent points and its centre, less the circular sector, all
in closed form."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeSolid
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.TopAbs import TopAbs_ShapeEnum
from OCP.TopoDS import TopoDS

from blend import edges_where, rounded
from edit import on_circle
from shell import hollow


def build():
    solid = BRepPrimAPI_MakeCone(6, 3, 6).Shape()
    solid = rounded(solid, 1.5, edges_where(solid, on_circle((0, 0, 6), 3)) + edges_where(solid, on_circle((0, 0, 0), 6)), "the rims")
    result = hollow(solid, 2.0)
    # OCCT's offset may give the solid back as its closed shell; the solid is the one it bounds.
    if result.ShapeType() == TopAbs_ShapeEnum.TopAbs_SHELL:
        result = BRepBuilderAPI_MakeSolid(TopoDS.Shell(result)).Solid()
    return result
