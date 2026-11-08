"""Two slabs united along flat B-spline faces in one plane. Two slabs united, touching along the
plane z = 4, where each has a face that is a B-spline patch lying flat in that plane. The two
patches are parametrised differently, and they overlap in part.

The first slab is the box from (0, 0, 0) to (10, 10, 4). Its top is a non-rational bicubic Bézier
patch (degree 3 both ways, knot vectors (0, 0, 0, 0, 1, 1, 1, 1)) whose 4 × 4 control points are
(xᵢ, yⱼ, 4) with x₀ … x₃ = 0, 2, 7, 10 and y₀ … y₃ = 0, 4, 5, 10. Its other five faces are planes.

The second slab is the box from (5, 3, 4) to (15, 13, 7). Its bottom is a non-rational biquadratic
Bézier patch (degree 2 both ways, knot vectors (0, 0, 0, 1, 1, 1)) whose 3 × 3 control points are
(xᵢ, yⱼ, 4) with x₀ … x₂ = 5, 12, 15 and y₀ … y₂ = 3, 4, 13. Its other five faces are planes.

Every control point of each patch lies in the plane z = 4, and each patch's x and y rise steadily
with its parameters, so each patch is exactly its square of that plane. The union is one solid; the
two patches overlap on the rectangle 5 ≤ x ≤ 10, 3 ≤ y ≤ 10."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_Sewing)
from OCP.BRepLib import BRepLib
from OCP.Geom import Geom_BezierSurface
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.collections import Array2_gp_Pnt
from OCP.gp import gp_Pnt

from bench import Refused

PLANE = 4  # the height both patches lie at
# the first slab, the box from (0, 0, 0) to (10, 10, 4), its top a bicubic patch
FIRST_LOW, FIRST_HIGH = (0, 0, 0), (10, 10, 4)
FIRST_XS, FIRST_YS = (0, 2, 7, 10), (0, 4, 5, 10)
# the second, the box from (5, 3, 4) to (15, 13, 7), its bottom a biquadratic patch
SECOND_LOW, SECOND_HIGH = (5, 3, 4), (15, 13, 7)
SECOND_XS, SECOND_YS = (5, 12, 15), (3, 4, 13)


def patch(xs, ys):
    """The flat Bézier patch whose control points are (xᵢ, yⱼ, 4): degree len − 1 both ways."""
    poles = Array2_gp_Pnt(1, len(xs), 1, len(ys))
    for i, x in enumerate(xs):
        for j, y in enumerate(ys):
            poles.SetValue(i + 1, j + 1, gp_Pnt(x, y, PLANE))
    return BRepBuilderAPI_MakeFace(Geom_BezierSurface(poles), 1e-9).Face()


def slab(low, high, patched):
    """The box from `low` to `high` with its face in the plane z = 4 the face `patched`, and its
    other five faces flat: the six faces sewn into a shell and closed into a solid."""
    (x0, y0, z0), (x1, y1, z1) = low, high
    corners = {(i, j, k): gp_Pnt((x0, x1)[i], (y0, y1)[j], (z0, z1)[k])
               for i in (0, 1) for j in (0, 1) for k in (0, 1)}
    loops = [[(i, 0, 0), (i, 1, 0), (i, 1, 1), (i, 0, 1)] for i in (0, 1)]
    loops += [[(0, j, 0), (1, j, 0), (1, j, 1), (0, j, 1)] for j in (0, 1)]
    loops += [[(0, 0, k), (1, 0, k), (1, 1, k), (0, 1, k)] for k in (0, 1)]
    sewing = BRepBuilderAPI_Sewing(1e-7)
    sewing.Add(patched)
    for loop in loops:
        if all(corners[c].Z() == PLANE for c in loop):
            continue  # the patch is this face
        polygon = BRepBuilderAPI_MakePolygon(*(corners[c] for c in loop), True)
        sewing.Add(BRepBuilderAPI_MakeFace(polygon.Wire()).Face())
    sewing.Perform()
    shells = TopExp_Explorer(sewing.SewedShape(), TopAbs_SHELL)
    if not shells.More():
        raise Refused("BRepBuilderAPI_Sewing made no shell of a slab's faces")
    solid = BRepBuilderAPI_MakeSolid(TopoDS.Shell(shells.Current()))
    if not solid.IsDone():
        raise Refused("BRepBuilderAPI_MakeSolid declined a slab's sewn shell")
    solid = solid.Solid()
    BRepLib.OrientClosedSolid_s(solid)
    return solid


def build():
    first = slab(FIRST_LOW, FIRST_HIGH, patch(FIRST_XS, FIRST_YS))
    second = slab(SECOND_LOW, SECOND_HIGH, patch(SECOND_XS, SECOND_YS))
    # the union must find that the two patches lie in one plane and share part of it
    fuse = BRepAlgoAPI_Fuse(first, second)
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two slabs")
    return fuse.Shape()
