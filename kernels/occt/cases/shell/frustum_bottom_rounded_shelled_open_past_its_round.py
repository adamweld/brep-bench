"""A cone frustum with its bottom rim rounded, shelled open at its top past the round's radius. A cone frustum on the z axis, from the disk of radius 6 at z = 0 to the disk of radius 3 at z = 6 (its side
the cone 2r + z = 12), with its bottom rim rounded to radius 1.5, each round a piece of a torus tangent to the flat face and to
the cone. Shell it inward to walls 2 thick, open at its top: every point of it within 2 of its faces other than the top. The cavity is every point at least 2 from the bottom and the cone, below the top: the frustum from z = 2 up to the top, z = 6, its side the cone 2r + z = 12 − 2√5, which comes out through the top in a circle of radius 0.7639320225. The walls are thicker than the round's radius, so the round moved inward turns inside out and drops, and the cavity has the sharp frustum's corners.

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
from bench import face_through
from shell import shell


def build():
    solid = BRepPrimAPI_MakeCone(6, 3, 6).Shape()
    solid = rounded(solid, 1.5, edges_where(solid, on_circle((0, 0, 0), 6)), "the rims")
    result = shell(solid, [face_through(solid, (0, 0, 6))], 2.0)
    # OCCT's offset may give the solid back as its closed shell; the solid is the one it bounds.
    if result.ShapeType() == TopAbs_ShapeEnum.TopAbs_SHELL:
        result = BRepBuilderAPI_MakeSolid(TopoDS.Shell(result)).Solid()
    return result
