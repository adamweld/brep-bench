"""What the OCCT implementations of the draft cases share: drafting faces of a solid with
BRepOffsetAPI_DraftAngle, a box with its vertical edges rounded, and a box's sides picked.

DraftAngle turns each face it is given about its neutral curve, where the face crosses the neutral
plane, until it makes the given angle with the pull direction: a plane turns about a line, and a
cylinder whose axis is the pull direction becomes a cone through the same circle. A positive angle
leans the face toward the solid's inside as it runs along the pull direction (an outer side
narrows, a hole's wall widens), and a negative one away from it. The faces beside the drafted ones
are extended or trimmed to meet them.
"""
import math

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepOffsetAPI import BRepOffsetAPI_DraftAngle
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeHalfSpace
from OCP.ShapeUpgrade import ShapeUpgrade_UnifySameDomain
from OCP.TopAbs import TopAbs_EDGE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Dir, gp_Pln, gp_Pnt

from bench import Refused, face_through


def drafted(solid, faces, degrees, neutral=(0, 0, 0), pull=(0, 0, 1)):
    """`solid` with each of `faces` drafted `degrees` about the plane through `neutral` square to
    `pull`. Refused if OCCT declines a face or the whole."""
    direction = gp_Dir(*pull)
    plane = gp_Pln(gp_Pnt(*neutral), direction)
    draft = BRepOffsetAPI_DraftAngle(solid)
    for k, face in enumerate(faces):
        draft.Add(face, direction, math.radians(degrees), plane)
        if not draft.AddDone():
            raise Refused(f"BRepOffsetAPI_DraftAngle declined face {k + 1} of {len(faces)} "
                          f"({status(draft)})")
    draft.Build()
    if not draft.IsDone():
        raise Refused("BRepOffsetAPI_DraftAngle could not build the drafted solid "
                      f"({status(draft)})")
    return draft.Shape()


def status(draft):
    """What DraftAngle could not recompute, from its Draft_ErrorStatus: a face, an edge or a
    vertex (`FaceRecomputation` and the like)."""
    return draft.Status().name.removeprefix("Draft_")


def rounded_box(lo, hi, radius):
    """The box from `lo` to `hi` with its four vertical edges, those whose ends differ only in z,
    rounded to `radius`."""
    box = BRepPrimAPI_MakeBox(gp_Pnt(*lo), gp_Pnt(*hi)).Shape()
    rounds = BRepFilletAPI_MakeFillet(box)
    edges = TopExp_Explorer(box, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        curve = BRepAdaptor_Curve(edge)
        a, b = curve.Value(curve.FirstParameter()), curve.Value(curve.LastParameter())
        if abs(a.X() - b.X()) < 1e-9 and abs(a.Y() - b.Y()) < 1e-9:
            rounds.Add(radius, edge)
        edges.Next()
    rounds.Build()
    if not rounds.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the vertical edges")
    return rounds.Shape()


def box_sides(solid, lo, hi, radius=0):
    """The faces of `solid` where the box from `lo` to `hi` has its four sides, picked halfway up
    each: its planes at their middles and, where its vertical edges are rounded to `radius`, the
    four rounds at theirs. A pocket's walls are picked by its box the same way."""
    (x0, y0, z0), (x1, y1, z1) = lo, hi
    xm, ym, z = (x0 + x1) / 2, (y0 + y1) / 2, (z0 + z1) / 2
    points = [(x0, ym, z), (x1, ym, z), (xm, y0, z), (xm, y1, z)]
    if radius:
        k = radius * (1 - 2 ** -0.5)  # a quarter round's middle, in from the box's corner
        points += [(x0 + k, y0 + k, z), (x1 - k, y0 + k, z),
                   (x1 - k, y1 - k, z), (x0 + k, y1 - k, z)]
    return [face_through(solid, p) for p in points]


def leaning_boxes_fused(rectangles, height, degrees):
    """The boxes over `rectangles` (each ((x0, y0), (x1, y1)) in the plane z = 0, `height` tall),
    each cut by the four planes through its floor's edges that lean `degrees` from z over it (the
    planes its sides would be drafted to; DraftAngle itself declines once they close), fused into
    one solid with the faces that lie in one plane made one. A block whose outline is the
    rectangles' union, its sides drafted until they close, is that solid where each concave corner
    of the outline is a corner of two of the rectangles: under the sides' planes its section at
    every height is the union of the rectangles' sections."""
    lean = math.tan(math.radians(degrees))
    whole = None
    for (x0, y0), (x1, y1) in rectangles:
        roof = BRepPrimAPI_MakeBox(gp_Pnt(x0, y0, 0), gp_Pnt(x1, y1, height)).Shape()
        inside = gp_Pnt((x0 + x1) / 2, (y0 + y1) / 2, 0)
        # each side's plane: through its floor edge, its normal out of the box and up by the lean
        for point, out in [((x0, y0, 0), (-1, 0)), ((x1, y0, 0), (1, 0)),
                           ((x0, y0, 0), (0, -1)), ((x0, y1, 0), (0, 1))]:
            plane = gp_Pln(gp_Pnt(*point), gp_Dir(out[0], out[1], lean))
            under = BRepPrimAPI_MakeHalfSpace(BRepBuilderAPI_MakeFace(plane).Face(), inside).Solid()
            cut = BRepAlgoAPI_Common(roof, under)
            cut.Build()
            if not cut.IsDone():
                raise Refused("BRepAlgoAPI_Common declined a leaning side")
            roof = cut.Shape()
        if whole is None:
            whole = roof
            continue
        fuse = BRepAlgoAPI_Fuse(whole, roof)
        fuse.Build()
        if not fuse.IsDone():
            raise Refused("BRepAlgoAPI_Fuse declined the roofs")
        whole = fuse.Shape()
    unify = ShapeUpgrade_UnifySameDomain(whole, True, True, False)
    unify.Build()
    return unify.Shape()


def united(base, boss):
    """`base` fused with `boss`."""
    fuse = BRepAlgoAPI_Fuse(base, boss)
    fuse.Build()
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the post")
    return fuse.Shape()


def turned_block(axis, rho0, rho1, z0, z1, degrees):
    """The rectangle between radii `rho0` and `rho1` from the vertical line through `axis` (x, y),
    and heights `z0` and `z1`, in the vertical plane running from the line toward +x, turned
    `degrees` counterclockwise (seen from +z) about the line."""
    from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
    from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
    from OCP.gp import gp_Ax1
    x, y = axis
    wire = BRepBuilderAPI_MakePolygon()
    for r, z in [(rho0, z0), (rho1, z0), (rho1, z1), (rho0, z1)]:
        wire.Add(gp_Pnt(x + r, y, z))
    wire.Close()
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    return BRepPrimAPI_MakeRevol(face, gp_Ax1(gp_Pnt(x, y, 0), gp_Dir(0, 0, 1)),
                                 math.radians(degrees)).Shape()


def revolved_bezier_block(poles, floor_radius):
    """The solid turned a full turn about the z axis from the region under the quadratic or cubic
    Bézier arc with `poles` (radius, height) pairs, between the axis and radius `floor_radius`,
    above z = 0."""
    from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
    from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
    from OCP.Geom import Geom_BezierCurve
    from OCP.collections import Array1_gp_Pnt
    from OCP.gp import gp_Ax1
    arr = Array1_gp_Pnt(1, len(poles))
    for i, (r, z) in enumerate(poles):
        arr.SetValue(i + 1, gp_Pnt(r, 0, z))
    arc = BRepBuilderAPI_MakeEdge(Geom_BezierCurve(arr)).Edge()
    (r0, z0), (r1, z1) = poles[0], poles[-1]
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(arc)
    for a, b in [((r1, 0, z1), (r1, 0, 0)), ((r1, 0, 0), (r0, 0, 0)), ((r0, 0, 0), (r0, 0, z0))]:
        wire.Add(BRepBuilderAPI_MakeEdge(gp_Pnt(*a), gp_Pnt(*b)).Edge())
    face = BRepBuilderAPI_MakeFace(wire.Wire()).Face()
    return BRepPrimAPI_MakeRevol(face, gp_Ax1(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1))).Shape()


def frustum_sector():
    """The quarter of the frustum about the z axis, radius 20 at z = 0 to 12 at z = 20, with
    x ≥ 0 and y ≥ 0."""
    from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
    from OCP.gp import gp
    return BRepPrimAPI_MakeCone(gp.XOY_s(), 20, 12, 20, math.pi / 2).Shape()
