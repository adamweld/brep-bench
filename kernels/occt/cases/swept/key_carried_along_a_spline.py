"""A square key carried along a spline. A block, the box from (−6, −6, −8) to (36, 26, 0), with a
groove cut into its top by a square key carried along a spline.

The curve is the cubic Bézier curve in the plane z = 0 whose control points are (0, 0, 0), (10, 0,
0), (20, 20, 0) and (30, 20, 0), run from its first end to its last.

A frame rides along the curve: at each point its origin is the curve's point, its x axis the
curve's tangent there (pointing the way the curve runs), its z axis straight up and its y axis to
the left, so that at the curve's start it is the world's own frame. The tool is drawn in that
starting frame and carried rigidly by the frame as it runs the whole curve.

The key, so drawn, is the box from (−1.5, −0.8, −1.5) to (1.5, 0.8, 2.5): 3 long along the curve,
1.6 across it, its bottom 1.5 below the block's top.

Cut from the block everything the key passes through as the frame runs the whole curve.

The key turns as the curve turns, so no screw carries it, and it is no solid of revolution: its
contact changes from face to face and edge to edge along the way, and its corners dig past its
sides where the curve bends."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.Geom import Geom_BezierCurve
from OCP.Geom2d import Geom2d_BezierCurve
from OCP.GeomLProp import GeomLProp_CurAndInf2d
from OCP.collections import Array1_gp_Pnt, Array1_gp_Pnt2d
from OCP.gp import gp, gp_Ax3, gp_Dir, gp_Pnt, gp_Pnt2d, gp_Trsf, gp_Vec

from bench import Refused
from sweep import cut, leads, moved, piped, plane, split, united

BLOCK = ((-6, -6, -8), (36, 26, 0))
CONTROL = [(0, 0), (10, 0), (20, 20), (30, 20)]  # in the plane z = 0
KEY = ((-1.5, -0.8, -1.5), (1.5, 0.8, 2.5))  # in the frame where the curve starts


def bezier(points):
    poles = Array1_gp_Pnt(1, len(points))
    for i, (x, y) in enumerate(points):
        poles.SetValue(i + 1, gp_Pnt(x, y, 0))
    return Geom_BezierCurve(poles)


def frame(curve, t):
    """The frame riding the curve at parameter t, as the motion that carries the start's frame
    (the world's) to it: its origin the curve's point, x along the tangent, z up."""
    p, d = gp_Pnt(), gp_Vec()
    curve.D1(t, p, d)
    trsf = gp_Trsf()
    trsf.SetDisplacement(gp_Ax3(gp.XOY_s()), gp_Ax3(p, gp.DZ_s(), gp_Dir(d)))
    return trsf, p, d


def build():
    curve = bezier(CONTROL)
    # The curve turns left and then right: where it inflects, the frame stops turning one way
    # and starts turning the other (OCCT finds the parameter, GeomLProp_CurAndInf2d).
    poles = Array1_gp_Pnt2d(1, len(CONTROL))
    for i, p in enumerate(CONTROL):
        poles.SetValue(i + 1, gp_Pnt2d(*p))
    inflections = GeomLProp_CurAndInf2d()
    inflections.PerformInf(Geom2d_BezierCurve(poles))
    if not inflections.IsDone():
        raise Refused("GeomLProp_CurAndInf2d could not find where the curve inflects")
    knots = [0] + sorted(inflections.Parameter(i + 1)
                         for i in range(inflections.NbPoints())) + [1]
    key = BRepPrimAPI_MakeBox(gp_Pnt(*KEY[0]), gp_Pnt(*KEY[1])).Shape()
    pieces = [key]
    for t0, t1 in zip(knots, knots[1:]):
        # A point at q in the frame moves, in the frame, with velocity v x̂ + ω ẑ × q, v the
        # curve's speed and ω the rate the frame turns. On the faces x = ±1.5 that is
        # ±(v − ω q_y), never zero: the curve never turns tighter than about 7.5, far more than
        # the key's half-width 0.8, so they lead and trail all over. On the faces y = ±0.8 it is
        # ±ω q_x: zero along the line x = 0, the plane through the curve's point square to its
        # tangent, and leading on the side the frame turns towards; between inflections ω keeps
        # its sign, so each span between them has one leading half of each. The level faces
        # slide and turn in their own planes.
        to_t0, at, along = frame(curve, t0)
        to_t1, _, _ = frame(curve, t1)
        start = moved(key, to_t0)
        p, d1, d2 = gp_Pnt(), gp_Vec(), gp_Vec()
        curve.D2((t0 + t1) / 2, p, d1, d2)
        turning = d1.Crossed(d2).Z() / d1.SquareMagnitude()  # ω per unit of the parameter

        def velocity(q, at=at, along=along, turning=turning):
            return along.Added(gp_Vec(0, 0, turning).Crossed(gp_Vec(at, q)))
        faces = [f for f in split(start, [plane(at.Coord(), along.Coord())])
                 if leads(f, velocity) > 0]
        span = Geom_BezierCurve(curve.Poles())
        span.Segment(t0, t1)
        spine = BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(span).Edge()).Wire()
        onward = to_t1.Multiplied(to_t0.Inverted())  # from where the span starts to its end
        sheets = [piped(spine, [f], [moved(f, onward)]) for f in faces]
        pieces.append(united(sheets, "the key's faces' sweeps along a span"))
    return cut(BRepPrimAPI_MakeBox(gp_Pnt(*BLOCK[0]), gp_Pnt(*BLOCK[1])).Shape(), pieces)
