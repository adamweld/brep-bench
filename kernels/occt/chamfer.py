"""What the OCCT implementations of the two-distance chamfer cases on a ball's edges share: the
circular edge through a point, the faces on either side of it, and BRepFilletAPI_MakeChamfer's
two-distance chamfer, whose distances are chords from the edge in the plane square to it; and,
where that chamfer declines a setback that consumes a face, the solid cut by the chamfer's cone."""
from OCP.BRepAdaptor import BRepAdaptor_Surface
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeVertex)
from OCP.BRepExtrema import BRepExtrema_DistShapeShape
from OCP.BRepFilletAPI import BRepFilletAPI_MakeChamfer
from OCP.BRepPrimAPI import BRepPrimAPI_MakeRevol
from OCP.GeomAbs import GeomAbs_SurfaceType
from OCP.TopAbs import TopAbs_EDGE, TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp, gp_Pnt

from bench import Refused

TOLERANCE = 1e-7


def edge_through(solid, point):
    """The one edge of `solid` passing through `point`."""
    vertex = BRepBuilderAPI_MakeVertex(gp_Pnt(*point)).Vertex()
    found = []
    edges = TopExp_Explorer(solid, TopAbs_EDGE)
    while edges.More():
        edge = TopoDS.Edge(edges.Current())
        distance = BRepExtrema_DistShapeShape(vertex, edge)
        if (distance.IsDone() and distance.Value() < TOLERANCE
                and not any(edge.IsSame(e) for e in found)):
            found.append(edge)
        edges.Next()
    assert len(found) == 1, f"{len(found)} edges through {point}"
    return found[0]


def faces_of(solid, edge):
    """The faces of `solid` bounded by `edge`."""
    found = []
    faces = TopExp_Explorer(solid, TopAbs_FACE)
    while faces.More():
        face = TopoDS.Face(faces.Current())
        edges = TopExp_Explorer(face, TopAbs_EDGE)
        while edges.More():
            if edges.Current().IsSame(edge):
                found.append(face)
                break
            edges.Next()
        faces.Next()
    return found


def is_ball_about(face, centre):
    """Whether `face` lies on a sphere centred at `centre`."""
    surface = BRepAdaptor_Surface(face)
    if surface.GetType() != GeomAbs_SurfaceType.GeomAbs_Sphere:
        return False
    return surface.Sphere().Location().Distance(gp_Pnt(*centre)) < TOLERANCE


def chamfered(solid, point, first, d_first, d_other, what):
    """`solid` with the edge through `point` chamfered `d_first` on the face for which `first`
    holds and `d_other` on the other; Refused, naming `what`, if OCCT declines."""
    edge = edge_through(solid, point)
    faces = [f for f in faces_of(solid, edge) if first(f)]
    assert len(faces) == 1, f"{len(faces)} faces of the edge through {point} hold the test"
    chamfer = BRepFilletAPI_MakeChamfer(solid)
    chamfer.Add(d_first, d_other, edge, faces[0])
    chamfer.Build()
    if not chamfer.IsDone():
        raise Refused(f"BRepFilletAPI_MakeChamfer could not chamfer {what}")
    return chamfer.Shape()


def cut_beyond_line(solid, point, direction, what, reach=12):
    """`solid` less everything beyond a cone about the z axis: in the half-plane (ρ, z), the points
    to the right of the line through `point` along `direction`, out to `reach`, turned about the
    axis. Refused, naming `what`, if the cut declines."""
    (px, pz), (dx, dz) = point, direction
    beyond = lambda q: (q[0] - px) * dz - (q[1] - pz) * dx  # positive to the right of the line
    # The square 0 ≤ ρ ≤ reach, |z| ≤ reach, clipped to that side of the line.
    corners = [(0, -reach), (reach, -reach), (reach, reach), (0, reach)]
    section = []
    for a, b in zip(corners, corners[1:] + corners[:1]):
        (sa, sb) = (beyond(a), beyond(b))
        if sa >= 0:
            section.append(a)
        if (sa < 0) != (sb < 0):
            t = sa / (sa - sb)
            section.append((a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t))
    return cut_by_revolved(solid, section, what)


def cut_by_revolved(solid, section, what):
    """`solid` less the solid of revolution about the z axis whose half-section is the polygon
    `section`, corners (ρ, z) in order, its first and last on the axis; Refused, naming `what`, if
    the cut declines."""
    polygon = BRepBuilderAPI_MakePolygon()
    for rho, z in section:
        polygon.Add(gp_Pnt(rho, 0, z))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire(), True).Face()
    tool = BRepPrimAPI_MakeRevol(face, gp.OZ_s()).Shape()
    cut = BRepAlgoAPI_Cut(solid, tool)
    if not cut.IsDone():
        raise Refused(f"BRepAlgoAPI_Cut declined to cut {what}")
    return cut.Shape()


def filled_by_revolved(solid, section, what):
    """`solid` joined to the solid of revolution about the z axis whose half-section is the
    polygon `section`, corners (ρ, z) in order; Refused, naming `what`, if the join declines."""
    polygon = BRepBuilderAPI_MakePolygon()
    for rho, z in section:
        polygon.Add(gp_Pnt(rho, 0, z))
    polygon.Close()
    face = BRepBuilderAPI_MakeFace(polygon.Wire(), True).Face()
    fill = BRepPrimAPI_MakeRevol(face, gp.OZ_s()).Shape()
    joined = BRepAlgoAPI_Fuse(solid, fill)
    if not joined.IsDone():
        raise Refused(f"BRepAlgoAPI_Fuse declined to fill {what}")
    return joined.Shape()
