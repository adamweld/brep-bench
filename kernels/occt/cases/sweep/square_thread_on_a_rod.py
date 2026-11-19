"""A square thread wound on a rod, its inner face on the rod's side. A rod with a thread of square
section wound round it and joined to it, the thread's inner face lying on the rod's side.

The rod is a cylinder of radius 10 on the z axis, from z = 0 to z = 20.

The thread is the square 10 ≤ x ≤ 12, 3 ≤ z ≤ 4 in the plane y = 0, its inner side on the rod's
surface, screwed about the z axis: turned right-handed, from +x toward +y, while it rises 3 for
every turn, through four whole turns. It ends in the plane y = 0 again, as the square 10 ≤ x ≤ 12,
15 ≤ z ≤ 16.

Each point of the square keeps its distance from the axis. So the thread's top and bottom are
helicoids, its outside lies on the cylinder of radius 12, and its inside lies on the rod's side,
along a helical band 1 wide. Its turns are 3 apart, which leaves gaps of 2 between them. Its two
ends are flat squares. The rod and the thread are one solid."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.Geom import Geom_CylindricalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.gp import gp, gp_Ax3, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d

from bench import Refused

ROD_RADIUS, ROD_LENGTH = 10, 20
SQUARE = [(10, 3), (12, 3), (12, 4), (10, 4)]  # (x, z) in the plane y = 0
PITCH, TURNS = 3, 4


def helix(radius, z):
    """A helix about the z axis from (radius, 0, z), right-handed, rising PITCH a turn: a straight
    line in the (angle, height) parameters of a cylinder."""
    surface = Geom_CylindricalSurface(gp_Ax3(gp.XOY_s()), radius)
    line = Geom2d_Line(gp_Pnt2d(0, z), gp_Dir2d(2 * math.pi, PITCH))
    edge = BRepBuilderAPI_MakeEdge(line, surface, 0, TURNS * math.hypot(2 * math.pi, PITCH)).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9)
    return BRepBuilderAPI_MakeWire(edge).Wire()


def build():
    rod = BRepPrimAPI_MakeCylinder(ROD_RADIUS, ROD_LENGTH).Shape()
    profile = BRepBuilderAPI_MakePolygon()
    for x, z in SQUARE:
        profile.Add(gp_Pnt(x, 0, z))
    profile.Close()
    # The square screwed along the helix through its middle: with the binormal held to the z
    # axis, every frame along the helix is the first one screwed, so the square stays in a plane
    # through the axis.
    thread = BRepOffsetAPI_MakePipeShell(helix(11, 3.5))
    thread.SetMode(gp_Dir(0, 0, 1))
    thread.SetTolerance(1e-9, 1e-9, 1e-9)  # the helicoids fitted closely, not to the 1e-4 default
    thread.Add(profile.Wire(), False, False)
    thread.Build()
    if not thread.IsDone() or not thread.MakeSolid():
        raise Refused("BRepOffsetAPI_MakePipeShell could not screw the square into a thread")
    joined = BRepAlgoAPI_Fuse(rod, thread.Shape())
    if not joined.IsDone():
        raise Refused("BRepAlgoAPI_Fuse could not join the thread to the rod")
    return joined.Shape()
