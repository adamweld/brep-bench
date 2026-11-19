"""A conical spring: round wire along a tapering helix. A conical compression spring: a round wire
of radius 1.5 coiled along a helix that tapers as it rises.

The wire's centre line is the helix

c(t) = ((20 − t/π) cos t, (20 − t/π) sin t, 4t/π),  0 ≤ t ≤ 8π:

it starts at (20, 0, 0), turns right-handed about the z axis (from +x toward +y) through four whole
turns, rising 8 each turn and closing in by 2 each turn, so it lies on a cone and ends at (12, 0,
32). The wire's section is the disk of radius 1.5 centred on the helix and square to it there,
carried along the helix with it, and its two ends are flat, square to the helix at its two ends.

Neighbouring coils are about 8.2 apart, centre to centre, so the wire never touches itself, and the
helix bends no tighter than a radius of about 12, far more than the wire's radius."""
import math

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Geom import Geom_ConicalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.gp import gp, gp_Ax2, gp_Ax3, gp_Circ, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d, gp_Vec

from bench import Refused

START_RADIUS = 20
PITCH = 8  # rise per turn
CLOSE_IN = 2  # fall in radius per turn
TURNS = 4
WIRE_RADIUS = 1.5


def helix():
    """The tapering helix as OCCT draws one: a straight line in the (angle, generator) parameters
    of a cone. The cone's frame is gp.XOY, so angle 0 is +x and the angle turns toward +y; its
    radius is START_RADIUS at z = 0 and falls CLOSE_IN for every PITCH it rises, so its generator
    leans in by atan(CLOSE_IN/PITCH) and runs √(PITCH² + CLOSE_IN²) a turn."""
    lean = math.atan2(CLOSE_IN, PITCH)
    cone = Geom_ConicalSurface(gp_Ax3(gp.XOY_s()), -lean, START_RADIUS)
    along = math.hypot(PITCH, CLOSE_IN)  # along the generator, each turn
    line = Geom2d_Line(gp_Pnt2d(0, 0), gp_Dir2d(2 * math.pi, along))
    length = TURNS * math.hypot(2 * math.pi, along)  # of the line, in the parameter plane
    edge = BRepBuilderAPI_MakeEdge(line, cone, 0, length).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9)  # the helix in space, from the line on the cone
    return edge


def build():
    path = helix()
    # The section where the helix starts: a circle centred on it, square to it.
    start, tangent = gp_Pnt(), gp_Vec()
    curve = BRepAdaptor_Curve(path)
    curve.D1(curve.FirstParameter(), start, tangent)
    section = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(start, gp_Dir(tangent)), WIRE_RADIUS)).Edge()

    pipe = BRepOffsetAPI_MakePipeShell(BRepBuilderAPI_MakeWire(path).Wire())
    pipe.SetMode(True)  # Frenet: the section keeps square to the helix
    pipe.Add(BRepBuilderAPI_MakeWire(section).Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the wire along the helix")
    if not pipe.MakeSolid():  # close the ends with flat disks
        raise Refused("the swept wire did not close into a solid")
    return pipe.Shape()
