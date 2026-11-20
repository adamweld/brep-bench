"""What the OCCT implementations of the freeform pipe cases share: sections made of B-spline and
Bezier curves, placed at the start of a path, and a pipe shell that sweeps them to a solid."""
from OCP.BRepAdaptor import BRepAdaptor_CompCurve
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Geom import Geom_BezierCurve, Geom_BSplineCurve
from OCP.collections import Array1_double, Array1_gp_Pnt, Array1_int
from OCP.gp import gp_Dir, gp_Pnt, gp_Vec

from bench import Refused


def array(kind, values):
    a = kind(1, len(values))
    for i, v in enumerate(values):
        a.SetValue(i + 1, v)
    return a


def at(center, u, v, a, b):
    """The point center + a*u + b*v."""
    return gp_Pnt(center.X() + a * u.X() + b * v.X(), center.Y() + a * u.Y() + b * v.Y(),
                  center.Z() + a * u.Z() + b * v.Z())


def periodic_square(center, u, v, half=3):
    """The periodic uniform quadratic B-spline whose control polygon is the square of corners
    center +- half*u +- half*v (taken in order round it), a closed curve, as a wire."""
    corners = [(half, half), (-half, half), (-half, -half), (half, -half)]
    poles = array(Array1_gp_Pnt, [at(center, u, v, a, b) for a, b in corners])
    knots = array(Array1_double, list(range(len(corners) + 1)))
    mults = array(Array1_int, [1] * (len(corners) + 1))
    curve = Geom_BSplineCurve(poles, knots, mults, 2, True)
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(curve).Edge()).Wire()


def closed_bezier(center, u, v, points):
    """The Bezier curve of the control points `points` (in (u, v) coordinates about `center`),
    the first and last the same, as a wire."""
    poles = array(Array1_gp_Pnt, [at(center, u, v, a, b) for a, b in points])
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(Geom_BezierCurve(poles)).Edge()).Wire()


def start_frame(path):
    """The point where the path starts and its unit tangent there."""
    first = BRepAdaptor_CompCurve(path)
    point, tangent = gp_Pnt(), gp_Vec()
    first.D1(first.FirstParameter(), point, tangent)
    return point, gp_Dir(tangent)


def pipe(path_wire, section, binormal=None, frenet=False, corners=None):
    """`section` swept along `path_wire` as a solid. With a `binormal` that direction is held fixed
    in the section; with `frenet` the Frenet trihedron carries it."""
    shell = BRepOffsetAPI_MakePipeShell(path_wire)
    if binormal is not None:
        shell.SetMode(binormal)
    elif frenet:
        shell.SetMode(True)
    if corners is not None:
        shell.SetTransitionMode(corners)
    shell.Add(section)
    shell.Build()
    if not shell.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the section")
    if not shell.MakeSolid():
        raise Refused("the swept section did not close into a solid")
    return shell.Shape()


def planar_pipe(edge):
    """The periodic B-spline square (half-side 3) swept along `edge`, a curve in the xy plane: its
    center on the path, one axis along z, the other in the plane square to the path and level."""
    path = BRepBuilderAPI_MakeWire(edge).Wire()
    start, tangent = start_frame(path)
    up = gp_Dir(0, 0, 1)
    across = up.Crossed(tangent)
    return pipe(path, periodic_square(start, across, up), binormal=up)


def tube_round_two_arcs(r1, r2, d, tube):
    """A circular pipe of radius `tube` along the outline of two overlapping disks, of radii r1 and
    r2 about (0, 0) and (d, 0): the arc of the first outside the second and the arc of the second
    outside the first, meeting at two corners, the corners made round. The path starts at the
    leftmost point of the first circle, (−r1, 0, 0), going down."""
    import math
    from OCP.BRepBuilderAPI import BRepBuilderAPI_TransitionMode
    from OCP.gp import gp_Ax2, gp_Circ
    x0 = (d * d + r1 * r1 - r2 * r2) / (2 * d)
    y0 = math.sqrt(r1 * r1 - x0 * x0)
    a = math.atan2(y0, x0)               # the corner's angle about the first center
    b = math.atan2(y0, x0 - d)           # and about the second
    first = gp_Circ(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), r1)
    second = gp_Circ(gp_Ax2(gp_Pnt(d, 0, 0), gp_Dir(0, 0, 1)), r2)
    wire = BRepBuilderAPI_MakeWire()
    wire.Add(BRepBuilderAPI_MakeEdge(first, math.pi, 2 * math.pi - a).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(second, -b, b).Edge())
    wire.Add(BRepBuilderAPI_MakeEdge(first, a, math.pi).Edge())
    path = wire.Wire()
    start, tangent = start_frame(path)
    ring = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(start, tangent), tube)).Edge()
    return pipe(path, BRepBuilderAPI_MakeWire(ring).Wire(),
                corners=BRepBuilderAPI_TransitionMode.BRepBuilderAPI_RoundCorner)
