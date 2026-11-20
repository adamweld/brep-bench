"""A rod with a helical groove cut by a bar whose outer face lies on the rod's side. A rod with a helical groove of rectangular section cut into its side, the groove's outer face lying on the rod's own cylinder.

The rod is the cylinder of radius 6 about the z axis from z = 0 to z = 20.

The groove is cut by the rectangle 4 ≤ x ≤ 6, 4 ≤ z ≤ 5.5 in the plane y = 0, its outer side on the rod's surface, screwed about the z axis: turned right-handed (from +x toward +y) while it rises 4 for every turn, through two and a half turns (900°). It ends in the plane y = 0 on the other side of the axis, as the rectangle −6 ≤ x ≤ −4, 14 ≤ z ≤ 15.5. Cut from the rod everything the rectangle passes through.

Each point of the rectangle keeps its distance from the axis. So the groove's floor lies on the cylinder of radius 4, its walls are helicoids, and its mouth is a helical band 1.5 wide on the rod's side. Its turns are 4 apart while it is 1.5 wide, so they leave 2.5 of the rod's side between them. Its two ends are flat rectangles in the plane y = 0, and lie wholly inside the rod."""
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

PITCH, TURNS = 4, 2.5


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


ROD_RADIUS, ROD_LENGTH = 6, 20


def build():
    rod = BRepPrimAPI_MakeCylinder(ROD_RADIUS, ROD_LENGTH).Shape()
    groove = screwed([(4, 4), (6, 4), (6, 5.5), (4, 5.5)], 5, 4.75)
    cut = BRepAlgoAPI_Cut(rod, groove)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut could not cut the groove from the rod")
    return cut.Shape()
