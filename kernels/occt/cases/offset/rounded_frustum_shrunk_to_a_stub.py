"""A rounded cone frustum shrunk to a stub. A cone frustum on the z axis, from the disk of radius 6 at z = 0 to the disk of radius 3 at z = 6 (its side
the cone 2r + z = 12), with both its circular edges rounded to radius 1.5: the top rim by a piece of a torus tangent to the top and to the cone, the bottom rim by one tangent to the bottom and to the cone, each of tube radius 1.5. Shrink it by 2.5: the solid of every point at least 2.5 from the
frustum's outside.

That is more than the rounds' radius. A round moved 2.5 inward would be a torus of tube radius
1.5 − 2.5 < 0, turned inside out: it is no part of the result, and the moved cone and the moved top or
bottom run on past it until they meet in a sharp edge. The result is the frustum shrunk with sharp
edges: from z = 2.5 to z = 3.5, its side the cone 2r + z = 12 − 2.5√5 (the side moved 2.5 along its
normal), so its radius is (12 − 2.5√5 − 2.5)/2 at the bottom and (12 − 2.5√5 − 3.5)/2 at the top."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeSolid
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.TopAbs import TopAbs_ShapeEnum
from OCP.TopoDS import TopoDS

from blend import edges_where, rounded
from edit import on_circle
from offset import offset

DISTANCE = 2.5


def build():
    solid = BRepPrimAPI_MakeCone(6, 3, 6).Shape()
    solid = rounded(solid, 1.5, edges_where(solid, on_circle((0, 0, 6), 3)) + edges_where(solid, on_circle((0, 0, 0), 6)), "the rims")
    shrunk = offset(solid, -DISTANCE)
    # OCCT's offset gives this solid back as its closed shell; the solid is the one it bounds.
    if shrunk.ShapeType() == TopAbs_ShapeEnum.TopAbs_SHELL:
        shrunk = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shrunk)).Solid()
    return shrunk
