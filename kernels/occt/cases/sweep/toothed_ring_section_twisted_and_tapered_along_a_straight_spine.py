r"""A toothed ring section twisted and tapered along a straight spine. A gear-like ring section swept along a straight spine, turned a quarter turn and shrunk to three fifths of its size as it goes: a stub of a toothed ring that twists and tapers.

The spine is the z axis from z = 0 to z = 10. The section at z = 0 is a toothed ring with six teeth, in the xy plane about the z axis:

- its bore is the circle of radius 1.5;
- its outer boundary goes round once, counter-clockwise, in twelve pieces, for k = 0 … 5 the tooth centered at azimuth 60°·k: the tip, an arc of the circle of radius 4 from azimuth 60°·k − 12° to 60°·k + 12°; then the straight flank from the end of that arc, (4, 60°·k + 12°), to (3, 60°·k + 18°) (polar coordinates, radius then azimuth); then the root, an arc of the circle of radius 3 from azimuth 60°·k + 18° to 60°·k + 42°; then the straight flank from its end to the start of the next tip, (4, 60°·(k + 1) − 12°).

The ring is the region between the bore and the outer boundary, so a tooth stands 1 proud of its root and is 24° wide at its tip.

The section at height z is the section at z = 0 scaled about the z axis by s(z) = 1 − 0.04·z and turned about the z axis, right-handed (from +x toward +y), by 9°·z. Both laws are linear in z: the section at z = 5 is scaled to 0.8 and turned 45°, and the one at z = 10 to 0.6 and turned 90°. A point of the section at distance ρ from the axis and azimuth φ is therefore at distance ρ·s(z) and azimuth φ + 9°·z at height z. The solid is bounded by the flat section at z = 0, the flat section at z = 10 (scaled to 0.6 and turned 90°), and the surfaces swept by the bore, the twelve tips and roots (each an arc swept into a twisted, tapering surface), and the twelve flanks (each a line swept into a twisted ruled surface)."""

import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepFill import BRepFill_TypeOfContact
from OCP.BRepLib import BRepLib
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.GC import GC_MakeArcOfCircle
from OCP.Geom import Geom_CylindricalSurface
from OCP.Geom2d import Geom2d_Line
from OCP.GeomAbs import GeomAbs_Shape
from OCP.Law import Law_Linear
from OCP.gp import gp_Ax2, gp_Ax3, gp_Circ, gp_Dir, gp_Dir2d, gp_Pnt, gp_Pnt2d

from bench import Refused

HEIGHT = 10
TURN = math.radians(90)  # right-handed about z, foot to top
SCALE_AT_TOP = 0.6
BORE, TIP, ROOT = 1.5, 4.0, 3.0


def helix():
    """The guide that turns the section: a line on the cylinder of radius TIP, its angle growing
    steadily with height, starting on +x."""
    cylinder = Geom_CylindricalSurface(
        gp_Ax3(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1), gp_Dir(1, 0, 0)), TIP)
    du, dv = TURN / HEIGHT, 1
    line = Geom2d_Line(gp_Pnt2d(0, 0), gp_Dir2d(du, dv))
    edge = BRepBuilderAPI_MakeEdge(line, cylinder, 0, HEIGHT * math.hypot(du, dv)).Edge()
    BRepLib.BuildCurves3d_s(edge, 1e-9, GeomAbs_Shape.GeomAbs_C2, 14, 0)
    return BRepBuilderAPI_MakeWire(edge).Wire()


def polar(r, degrees):
    a = math.radians(degrees)
    return gp_Pnt(r * math.cos(a), r * math.sin(a), 0)


def toothed_outline():
    """The outer boundary: six tips (arcs of radius 4), six roots (arcs of radius 3), joined by
    twelve straight flanks."""
    wire = BRepBuilderAPI_MakeWire()
    for k in range(6):
        c = 60 * k
        tip = GC_MakeArcOfCircle(polar(TIP, c - 12), polar(TIP, c), polar(TIP, c + 12)).Value()
        root = GC_MakeArcOfCircle(polar(ROOT, c + 18), polar(ROOT, c + 30),
                                  polar(ROOT, c + 42)).Value()
        wire.Add(BRepBuilderAPI_MakeEdge(tip).Edge())
        wire.Add(BRepBuilderAPI_MakeEdge(polar(TIP, c + 12), polar(ROOT, c + 18)).Edge())
        wire.Add(BRepBuilderAPI_MakeEdge(root).Edge())
        wire.Add(BRepBuilderAPI_MakeEdge(polar(ROOT, c + 42), polar(TIP, c + 48)).Edge())
    if not wire.IsDone():
        raise Refused("BRepBuilderAPI_MakeWire could not close the toothed outline")
    return wire.Wire()


def bore_circle():
    circle = gp_Circ(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), BORE)
    return BRepBuilderAPI_MakeWire(BRepBuilderAPI_MakeEdge(circle).Edge()).Wire()


def swept(profile, name):
    """`profile` swept up the z axis, turned by the helix guide and scaled by a law running
    linearly from 1 at the foot to SCALE_AT_TOP at the top, as a solid."""
    axis = BRepBuilderAPI_MakeWire(
        BRepBuilderAPI_MakeEdge(gp_Pnt(0, 0, 0), gp_Pnt(0, 0, HEIGHT)).Edge()).Wire()
    law = Law_Linear()
    law.Set(0.0, 1.0, 1.0, SCALE_AT_TOP)  # by the fraction of the spine run
    pipe = BRepOffsetAPI_MakePipeShell(axis)
    pipe.SetMode(helix(), False, BRepFill_TypeOfContact.BRepFill_NoContact)
    pipe.SetLaw(profile, law, False, False)
    pipe.SetTolerance(1e-7, 1e-7, 1e-6)
    pipe.SetMaxDegree(14)
    pipe.Build()
    if not pipe.IsDone():
        raise Refused(f"BRepOffsetAPI_MakePipeShell could not sweep the {name} along its guide")
    if not pipe.MakeSolid():
        raise Refused(f"the swept {name} did not close into a solid")
    return pipe.Shape()


def build():
    # the toothed disk and the bore each swept the same way, the bore then cut out
    body = swept(toothed_outline(), "toothed outline")
    bore = swept(bore_circle(), "bore")
    cut = BRepAlgoAPI_Cut(body, bore)
    if not cut.IsDone():
        raise Refused("BRepAlgoAPI_Cut declined the swept bore")
    return cut.Shape()
