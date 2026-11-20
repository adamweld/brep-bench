"""A hollow tube whose section is a ring, screwed round the z axis. A tube whose section is a ring, standing clear of the z axis, screwed round it.

The section is the ring between the circles of radius 2 and radius 1 about the point (8, 0, 2) in the plane y = 0. It is screwed about the z axis: turned right-handed (from +x toward +y) while it rises 6 for every turn, through three whole turns, so that it ends in the plane y = 0 again as the ring about (8, 0, 20).

Each point of the section keeps its distance from the axis. So the tube's outside and its bore are surfaces swept by circles, its two ends are flat rings in the plane y = 0, and its turns, 6 apart while the ring is 4 high, leave gaps of 2 between them. It has no closed form for its area, though its volume is 3π · 2π · 8 · 3 = 144π²."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.Geom import Geom_ConicalSurface, Geom_CylindricalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.GC import GC_MakeCircle
from OCP.gp import gp, gp_Ax2, gp_Ax3, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d

from bench import Refused

def helix(radius, z, pitch, turns):
    """A helix about the z axis from (radius, 0, z), right-handed, rising pitch a turn: a straight
    line in the (angle, height) parameters of a cylinder."""
    surface = Geom_CylindricalSurface(gp_Ax3(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), radius)
    line = Geom2d_Line(gp_Pnt2d(0, z), gp_Dir2d(2 * math.pi, pitch))
    edge = BRepBuilderAPI_MakeEdge(line, surface, 0, turns * math.hypot(2 * math.pi, pitch)).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9)
    return BRepBuilderAPI_MakeWire(edge).Wire()


def pipe(profile, spine):
    """The profile carried along the spine with the binormal held to the z axis, as a solid."""
    sweep = BRepOffsetAPI_MakePipeShell(spine)
    sweep.SetMode(gp_Dir(0, 0, 1))
    sweep.SetTolerance(1e-9, 1e-9, 1e-9)
    sweep.Add(profile, False, False)
    sweep.Build()
    if not sweep.IsDone() or not sweep.MakeSolid():
        raise Refused("BRepOffsetAPI_MakePipeShell could not carry the profile along the path")
    return sweep.Shape()


def circle(radius, cx, cz):
    """The circle of the given radius about (cx, 0, cz) in the plane y = 0."""
    arc = GC_MakeCircle(gp_Ax2(gp_Pnt(cx, 0, cz), gp_Dir(0, 1, 0)), radius).Value()
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(arc).Edge()).Wire()


PITCH, TURNS = 6, 3


def build():
    spine = helix(8, 2, PITCH, TURNS)
    outer = pipe(circle(2, 8, 2), spine)
    bore = pipe(circle(1, 8, 2), spine)
    cut = BRepAlgoAPI_Cut(outer, bore)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not bore the tube")
    return cut.Shape()
