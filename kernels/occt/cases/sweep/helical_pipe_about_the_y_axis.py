"""A helical pipe wound about the y axis. A pipe of circular section, radius 1, following a helix
whose axis is the y axis, not the z axis. The helix has radius 8 and rises 3 per turn, along +y. It
starts at (0, 0, 8), turns right-handed about +y (from +z toward +x) and makes two and a half turns,
so it ends at (0, 7.5, -8). The section stays square to the path, and the pipe's ends are flat,
square to the helix there."""
import math

from OCP.BRepAdaptor import BRepAdaptor_Curve
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Geom import Geom_CylindricalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.gp import gp, gp_Ax2, gp_Ax3, gp_Circ, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d, gp_Vec

from bench import Refused

HELIX_RADIUS = 8
PITCH = 3  # rise per turn
TURNS = 2.5
PIPE_RADIUS = 1


def helix():
    """The helix as OCCT draws one: a straight line in the (angle, height) parameters of a
    cylinder. The cylinder's frame has its axis along +y and its x direction along +z, so angle 0
    is +z and the angle turns towards +x; each turn, the line climbs 2π in angle and PITCH in height."""
    frame = gp_Ax3(gp_Pnt(0, 0, 0), gp_Dir(0, 1, 0), gp_Dir(0, 0, 1))
    cylinder = Geom_CylindricalSurface(frame, HELIX_RADIUS)
    line = Geom2d_Line(gp_Pnt2d(0, 0), gp_Dir2d(2 * math.pi, PITCH))
    length = TURNS * math.hypot(2 * math.pi, PITCH)  # of the line, in the parameter plane
    edge = BRepBuilderAPI_MakeEdge(line, cylinder, 0, length).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9)  # the helix in space, from the line on the cylinder
    return edge


def build():
    path = helix()
    # The section where the helix starts: a circle centred on the helix, square to it.
    start, tangent = gp_Pnt(), gp_Vec()
    curve = BRepAdaptor_Curve(path)
    curve.D1(curve.FirstParameter(), start, tangent)
    section = BRepBuilderAPI_MakeEdge(gp_Circ(gp_Ax2(start, gp_Dir(tangent)), PIPE_RADIUS)).Edge()

    pipe = BRepOffsetAPI_MakePipeShell(BRepBuilderAPI_MakeWire(path).Wire())
    pipe.SetMode(True)  # Frenet: the section keeps square to the helix
    pipe.Add(BRepBuilderAPI_MakeWire(section).Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the circle along the helix")
    if not pipe.MakeSolid():  # close the ends with flat discs
        raise Refused("the swept pipe did not close into a solid")
    return pipe.Shape()
