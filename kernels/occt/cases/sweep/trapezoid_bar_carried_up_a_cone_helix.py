"""A bar with a side along a cone carried up a helix that widens as it climbs. A bar whose section has one side slanted to follow a cone, carried up a helix about the z axis whose radius grows as it climbs, at the cone's angle.

The path c(θ) starts at (5, 0, 2) and turns right-handed (from +x toward +y) through three whole turns: for each turn it rises 6 and its distance from the axis grows by 1.5, so c(θ) = (ρ cos θ, ρ sin θ, z) with ρ = 5 + 1.5 θ/360° and z = 2 + 6 θ/360°, for 0 ≤ θ ≤ 1080°. It lies on a cone about the z axis whose side leans at arctan(1/4) to the axis.

The section is the quadrilateral with corners (4, 0, 0), (6, 0, 0), (7, 0, 4) and (4, 0, 4) in the plane y = 0: its outer side from (6, 0, 0) to (7, 0, 4) leans at the cone's angle, arctan(1/4) to the vertical.

Each section is carried along the path by the path's own frame, with the z axis held in it: at each point of the path, T is the unit tangent (the full three-dimensional tangent of the path, with its outward lean, at the start as everywhere), B is the z axis with its component along T taken out (made unit), and N = B × T. The section keeps its coordinates in (T, N, B): a point p of the starting section goes to c(θ) + R(θ)(p − c(0)), where c(θ) is the point of the path and R(θ) the rotation taking the starting frame to the frame at c(θ). On a cylinder this is a plain screw and the section stays in a plane through the axis; on the cone the path's tangent leans outward as it climbs, so the section leans with it, and does not stay in a plane through the axis.

The bar's ends are flat quadrilaterals. Its turns are 6 apart in height while the section is 4 high, so they do not touch. It has no closed form."""
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
    profile = BRepBuilderAPI_MakePolygon()
    for x, z in [(4, 0), (6, 0), (7, 4), (4, 4)]:
        profile.Add(gp_Pnt(x, 0, z))
    profile.Close()
    return pipe(profile.Wire(), cone_helix(5, 2, 6, 1.5, 3))
