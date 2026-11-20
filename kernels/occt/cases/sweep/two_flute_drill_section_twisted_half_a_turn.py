r"""A two-fluted drill section twisted half a turn. A twist-drill-like bar: a round section with two circular flutes cut out of it, turned steadily as it rises along a straight axis, half a turn in all.

The solid runs from z = 0 to z = 12. Its section at z = 0 is the disk of radius 3 about the z axis less two round flutes: the disks of radius 1.9 centered on (3.4, 0) and (−3.4, 0). What is left is one piece, a web 3 wide across its middle (|x| ≤ 1.5 at y = 0) with a lobe at each end. Its boundary is two arcs of the radius-3 circle, one at the top and one at the bottom (each about 112.4° round), and two concave arcs of the flute circles (each about 122.9° round), meeting in four corners at (±2.4926, ±1.6693), where the radius-3 circle meets a flute circle. The section at every height z is this one turned about the z axis, right-handed (from +x toward +y), by 15° for every unit of height: at z = 6 a quarter turn, and at z = 12 a half turn, which brings the section back onto itself.

The flutes' edges run along helices: a corner of the section at distance ρ from the axis runs on a helix of radius ρ that rises 24 in a turn. The solid is swept from the section along the straight axis, turned about it by that guide helix. Its two ends are flat, the section at z = 0 and its turned copy at z = 12; its other faces are two helicoidal strips of the radius-3 cylinder and the two flute surfaces, swept by the section's arcs.

**As a set:**

$$\{\,0 \le z \le 12,\ u^2 + v^2 \le 9,\ (u - 3.4)^2 + v^2 \ge 1.9^2,\ (u + 3.4)^2 + v^2 \ge 1.9^2\,\},\quad (u, v) = (x\cos\alpha + y\sin\alpha,\ -x\sin\alpha + y\cos\alpha),\ \alpha = 15^\circ z$$"""

import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepFill import BRepFill_TypeOfContact
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.GC import GC_MakeArcOfCircle
from OCP.Geom import Geom_CylindricalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.GeomAbs import GeomAbs_Shape
from OCP.gp import gp_Ax3, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d

from bench import Refused

HEIGHT = 12
TURN = math.pi  # right-handed about z, foot to top
R, FLUTE_R, FLUTE_D = 3.0, 1.9, 3.4


def helix():
    """The guide a point of the section at distance R from the axis, starting on +x, runs along:
    a line on the cylinder of radius R, its angle growing steadily with height."""
    cylinder = Geom_CylindricalSurface(
        gp_Ax3(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)), R)
    du, dv = TURN / HEIGHT, 1
    line = Geom2d_Line(gp_Pnt2d(0, 0), gp_Dir2d(du, dv))
    edge = BRepBuilderAPI_MakeEdge(line, cylinder, 0, HEIGHT * math.hypot(du, dv)).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9, GeomAbs_Shape.GeomAbs_C2, 14, 0)
    return BRepBuilderAPI_MakeWire(edge).Wire()


def section():
    """The disk of radius R less the two flute disks, as a wire of four circular arcs."""
    x0 = (FLUTE_D**2 + R**2 - FLUTE_R**2) / (2 * FLUTE_D)
    y0 = math.sqrt(R**2 - x0**2)
    p = lambda x, y: gp_Pnt(x, y, 0)
    arcs = [
        GC_MakeArcOfCircle(p(x0, y0), p(0, R), p(-x0, y0)).Value(),         # top lobe
        GC_MakeArcOfCircle(p(-x0, y0), p(-(FLUTE_D - FLUTE_R), 0), p(-x0, -y0)).Value(),  # left flute
        GC_MakeArcOfCircle(p(-x0, -y0), p(0, -R), p(x0, -y0)).Value(),      # bottom lobe
        GC_MakeArcOfCircle(p(x0, -y0), p(FLUTE_D - FLUTE_R, 0), p(x0, y0)).Value(),       # right flute
    ]
    wire = BRepBuilderAPI_MakeWire()
    for arc in arcs:
        wire.Add(BRepBuilderAPI_MakeEdge(arc).Edge())
    if not wire.IsDone():
        raise Refused("BRepBuilderAPI_MakeWire could not close the section")
    return wire.Wire()


def build():
    axis = BRepBuilderAPI_MakeWire(
        BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 0), gp_Pnt(0, 0, HEIGHT)).Edge()).Wire()
    # A sweep up the axis, the helix its auxiliary spine: at each height the section is turned
    # so that its x direction points at where the plane at that height meets the helix.
    pipe = BRepOffsetAPI_MakePipeShell(axis)
    pipe.SetMode(helix(), False, BRepFill_TypeOfContact.BRepFill_NoContact)
    pipe.Add(section(), False, False)
    pipe.SetTolerance(1e-7, 1e-7, 1e-6)
    pipe.SetMaxDegree(14)
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the section along its guide")
    if not pipe.MakeSolid():
        raise Refused("the swept section did not close into a solid")
    return pipe.Shape()
