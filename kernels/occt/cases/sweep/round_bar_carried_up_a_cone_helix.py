"""A round bar carried up a helix that widens as it climbs. A bar of circular section carried up a helix about the z axis whose radius grows as it climbs, a coil that opens like a cone.

The path c(θ) starts at (5, 0, 0) and turns right-handed (from +x toward +y) through three whole turns: for each turn it rises 4 and its distance from the axis grows by 1, so c(θ) = (ρ cos θ, ρ sin θ, z) with ρ = 5 + θ/360° and z = 4 θ/360°, for 0 ≤ θ ≤ 1080°. It lies on a cone about the z axis whose side leans at arctan(1/4) to the axis.

The section is the disc of radius 1 about (5, 0, 0) in the plane y = 0.

Each section is carried along the path by the path's own frame, with the z axis held in it: at each point of the path, T is the unit tangent (the full three-dimensional tangent of the path, with its outward lean, at the start as everywhere), B is the z axis with its component along T taken out (made unit), and N = B × T. The section keeps its coordinates in (T, N, B): a point p of the starting section goes to c(θ) + R(θ)(p − c(0)), where c(θ) is the point of the path and R(θ) the rotation taking the starting frame to the frame at c(θ). On a cylinder this is a plain screw and the section stays in a plane through the axis; on the cone the path's tangent leans outward as it climbs, so the section leans with it, and does not stay in a plane through the axis.

The bar's ends are flat discs. Its turns are 4 apart in height and 1 apart in radius while the disc is 2 wide, so they do not touch. It has no closed form."""
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

def pipe(profile, spine):
    """The profile carried along the spine with the binormal held to the z axis, as a solid.
    Known deviation: OCCT's binormal mode does not build the frame the case states (volume
    0.3% / 0.9% high, surfaces up to 1e-2 off); its Frenet mode is exact for the round bar only."""
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


def cone_helix(radius, z, rise, growth, turns):
    """A helix about the z axis from (radius, 0, z), right-handed, rising `rise` and growing in
    radius by `growth` each turn: a straight line in the (angle, distance along the generator)
    parameters of a cone."""
    half = math.atan2(growth, rise)
    surface = Geom_ConicalSurface(gp_Ax3(gp_Pnt(0, 0, z), gp_Dir(0, 0, 1)), half, radius)
    along = math.hypot(rise, growth)
    line = Geom2d_Line(gp_Pnt2d(0, 0), gp_Dir2d(2 * math.pi, along))
    edge = BRepBuilderAPI_MakeEdge(line, surface, 0, turns * math.hypot(2 * math.pi, along)).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9)
    return BRepBuilderAPI_MakeWire(edge).Wire()


def build():
    return pipe(circle(1, 5, 0), cone_helix(5, 0, 4, 1, 3))
