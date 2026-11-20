"""A coil of round wire in five helical stretches, narrowing in steps, each of its own pitch. A round
wire of radius 1 coiled about the z axis along a centre line of five helical stretches, one after
another: wide, closing in, narrow, opening out, wide again. Every stretch is a helix about the z
axis, turning right-handed (from +x toward +y), whose radius and height both change in proportion to
the angle turned. The coil starts at (12, 0, 0).

    stretch   turns   radius           rise     pitch
    1         1.5     12 throughout    7.5      5
    2         1.25    12 closing to 8  3.125    2.5
    3         2       8 throughout     12       6
    4         1       8 opening to 12  5        5
    5         0.75    12 throughout    3        4

Each stretch starts where the one before ends, with a different pitch, so the centre line has a
slight corner at each joint. The wire is the disk of radius 1, centred on the centre line and
square to it, carried along each stretch with it, with flat ends. At each joint the two stretches
are mitred."""
import math

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Geom import Geom_ConicalSurface, Geom_CylindricalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.gp import gp_Ax2, gp_Ax3, gp_Circ, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d, gp_Vec

from bench import Refused
from corner import MITRE

WIRE_RADIUS = 1
# (turns, radius at the start, radius at the end, rise)
STRETCHES = [(1.5, 12, 12, 7.5), (1.25, 12, 8, 3.125), (2, 8, 8, 12), (1, 8, 12, 5),
             (0.75, 12, 12, 3)]


def stretch(turns, r0, r1, rise, angle, height):
    """One stretch as OCCT draws a helix: a straight line in the (angle, along) parameters of a
    cylinder or, where the radius changes, a cone, whose frame is the z axis from `height` up. The
    line starts at `angle` and climbs 2π of angle each turn."""
    frame = gp_Ax3(gp_Pnt(0, 0, height), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0))
    if r0 == r1:
        surface = Geom_CylindricalSurface(frame, r0)
        along = rise / turns  # the height each turn
    else:
        per_turn = (r1 - r0) / turns  # the change in radius each turn
        pitch = rise / turns
        surface = Geom_ConicalSurface(frame, math.atan2(per_turn, pitch), r0)
        along = math.hypot(per_turn, pitch)  # along the generator, each turn
    line = Geom2d_Line(gp_Pnt2d(angle, 0), gp_Dir2d(2 * math.pi, along))
    length = turns * math.hypot(2 * math.pi, along)  # of the line, in the parameter plane
    edge = BRepBuilderAPI_MakeEdge(line, surface, 0, length).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9)
    return edge


def build():
    wire = BRepBuilderAPI_MakeWire()
    angle, height = 0.0, 0.0
    for turns, r0, r1, rise in STRETCHES:
        wire.Add(stretch(turns, r0, r1, rise, angle, height))
        angle += turns * 2 * math.pi
        height += rise
    if not wire.IsDone():
        raise Refused("the five helical stretches did not join into a wire")
    path = wire.Wire()

    # The section where the coil starts: a circle centred on it, square to it.
    start, tangent = gp_Pnt(), gp_Vec()
    first = BRepAdaptor_Curve(next(iter_edges(path)))
    first.D1(first.FirstParameter(), start, tangent)
    section = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(start, gp_Dir(tangent)), WIRE_RADIUS)).Edge()

    pipe = BRepOffsetAPI_MakePipeShell(path)
    pipe.SetMode(True)  # Frenet: the section keeps square to the path
    pipe.SetTransitionMode(MITRE)
    pipe.Add(BRepBuilderAPI_MakeWire(section).Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the wire along the five stretches")
    if not pipe.MakeSolid():
        raise Refused("the swept wire did not close into a solid")
    return pipe.Shape()


def iter_edges(wire):
    from OCP.BRepTools import BRepTools_WireExplorer
    from OCP.TopoDS import TopoDS
    walk = BRepTools_WireExplorer(wire)
    while walk.More():
        yield TopoDS.Edge(walk.Current())
        walk.Next()
