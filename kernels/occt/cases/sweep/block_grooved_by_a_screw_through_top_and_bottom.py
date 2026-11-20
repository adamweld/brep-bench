"""A block with a helical groove screwed through it from its top to its bottom. A block from
(-10, -10, 0) to (10, 10, 12), cut by the square 3 <= x <= 5, -3 <= z <= -1 in the plane y = 0
screwed about the z axis, right-handed, rising 4 a turn through four turns."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.Geom import Geom_CylindricalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.gp import gp, gp_Ax3, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d

from bench import Refused

PITCH, TURNS = 4, 4


def helix(radius, z):
    """A helix about the z axis from (radius, 0, z), right-handed, rising PITCH a turn: a straight
    line in the (angle, height) parameters of a cylinder."""
    surface = Geom_CylindricalSurface(gp_Ax3(gp.XOY_s()), radius)
    line = Geom2d_Line(gp_Pnt2d(0, z), gp_Dir2d(2 * math.pi, PITCH))
    edge = BRepBuilderAPI_MakeEdge(line, surface, 0, TURNS * math.hypot(2 * math.pi, PITCH)).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9)
    return BRepBuilderAPI_MakeWire(edge).Wire()


def screwed(corners, spine_radius, spine_z):
    """The profile (x, z) in the plane y = 0, carried along the helix through (spine_radius,
    spine_z) with the binormal held to the z axis: a screw."""
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
    block = BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, 0), gp_Pnt(10, 10, 12)).Shape()
    groove = screwed([(3, -3), (5, -3), (5, -1), (3, -1)], 4, -2)
    cut = BRepAlgoAPI_Cut(block, groove)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the groove from the block")
    return cut.Shape()
