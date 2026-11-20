"""A bar of rectangular section screwed round the z axis, clear of it. A bar of rectangular section, standing clear of the z axis, screwed round it.

The section is the rectangle 4 ≤ x ≤ 7, 0 ≤ z ≤ 2 in the plane y = 0. It is screwed about the z axis: turned right-handed (from +x toward +y) while it rises 5 for every turn, through three whole turns, so that it ends in the plane y = 0 again as the rectangle 4 ≤ x ≤ 7, 15 ≤ z ≤ 17.

Each point of the section keeps its distance from the axis. So the bar's inside lies on the cylinder of radius 4 and its outside on the cylinder of radius 7, its top and bottom are helicoids, and its two ends are flat rectangles in the plane y = 0. Its turns are 5 apart while the section is 2 high, so they leave gaps of 3 between them."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.Geom import Geom_CylindricalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.gp import gp, gp_Ax3, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d

from bench import Refused

PITCH, TURNS = 5, 3


def helix(radius, z):
    """A helix about the z axis from (radius, 0, z), right-handed, rising PITCH a turn: a straight
    line in the (angle, height) parameters of a cylinder."""
    surface = Geom_CylindricalSurface(gp_Ax3(gp.XOY_s()), radius)
    line = Geom2d_Line(gp_Pnt2d(0, z), gp_Dir2d(2 * math.pi, PITCH))
    edge = BRepBuilderAPI_MakeEdge(line, surface, 0, TURNS * math.hypot(2 * math.pi, PITCH)).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9)
    return BRepBuilderAPI_MakeWire(edge).Wire()


def screwed(corners, spine_radius, spine_z):
    """The rectangle (x, z) in the plane y = 0, carried along the helix through (spine_radius,
    spine_z) with the binormal held to the z axis: a screw, the profile staying in a plane through
    the axis."""
    profile = BRepBuilderAPI_MakePolygon()
    for x, z in corners:
        profile.Add(gp_Pnt(x, 0, z))
    profile.Close()
    sweep = BRepOffsetAPI_MakePipeShell(helix(spine_radius, spine_z))
    sweep.SetMode(gp_Dir(0, 0, 1))
    sweep.SetTolerance(1e-9, 1e-9, 1e-9)
    sweep.Add(profile.Wire(), False, False)
    sweep.Build()
    if not sweep.IsDone() or not sweep.MakeSolid():
        raise Refused("BRepOffsetAPI_MakePipeShell could not screw the profile")
    return sweep.Shape()


def build():
    return screwed([(4, 0), (7, 0), (7, 2), (4, 2)], 5.5, 1)
