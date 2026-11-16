"""A turned post with its shoulder rounded, shrunk by 2, past the round. A turned post on the z axis: the cylinder of radius 5 from z = 0 to z = 6, topped by the cone frustum from radius 5 at
z = 6 to radius 2 at z = 10 (its side the cone 4r + 3z = 38), joined. The convex edge between the cylinder and the cone,
the circle of radius 5 at z = 6, is rounded to radius 1: a piece of a torus tangent to both. Shrink it by 2: the solid
of every point at least 2 from its outside.

That is more than the round's radius. The round moved 2 inward would be a torus of tube radius 1 − 2 < 0, turned
inside out, and drops; the moved cylinder and cone meet in a sharp edge. The post is convex, and the rounded post lies
between the sharp post shrunk by 1 and grown back by 1 and the sharp post itself; both shrink by 2 to the same solid,
the sharp post's points at least 2 inside: its half-section the polygon (0, 2), (3, 2), (3, 5.33333333333),
(1, 8), (0, 8) turned about the axis (the base up 2, the side in 2, the cone in 2 along its
normal, 4r + 3z = 38 − 5·2, the top down 2)."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeSolid
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.TopAbs import TopAbs_ShapeEnum
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from blend import edges_where, rounded
from edit import on_circle
from offset import offset

DISTANCE = 2.0


def build():
    cone = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(0, 0, 6), gp_Dir(0, 0, 1)), 5, 2, 4).Shape()
    post = BRepAlgoAPI_Fuse(BRepPrimAPI_MakeCylinder(5, 6).Shape(), cone).Shape()
    solid = rounded(post, 1, edges_where(post, on_circle((0, 0, 6), 5)), "the shoulder")
    result = offset(solid, -DISTANCE)
    # OCCT's offset may give the solid back as its closed shell; the solid is the one it bounds.
    if result.ShapeType() == TopAbs_ShapeEnum.TopAbs_SHELL:
        result = BRepBuilderAPI_MakeSolid(TopoDS.Shell(result)).Solid()
    return result
