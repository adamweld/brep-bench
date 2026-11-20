"""A spur gear of twelve involute teeth: the flat outline (root circle 9, base circle 10, tip circle
12.5, each tooth 0.4 rad wide at the base circle) extruded 8 up from z = 0. The involute flanks are
splines through points of the involute."""
import math

from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace,
                                BRepBuilderAPI_MakeWire)
from OCP.BRepPrimAPI import BRepPrimAPI_MakePrism
from OCP.GeomAPI import GeomAPI_Interpolate
from OCP.collections import HArray1_gp_Pnt
from OCP.gp import gp_Ax2, gp_Circ, gp_Dir, gp_Pnt, gp_Vec

from bench import Refused

TEETH = 12
RB, RA, RF = 10.0, 12.5, 9.0
HALF = 0.2           # half-width of a tooth on the base circle, radians
HEIGHT = 8
ALPHA_TIP = math.acos(RB / RA)
SAMPLES = 40


def polar(r, angle):
    return gp_Pnt(r * math.cos(angle), r * math.sin(angle), 0)


def line(a, b):
    return BRepBuilderAPI_MakeEdge(a, b).Edge()


def arc(r, a0, a1):
    circle = gp_Circ(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, 1)), r)
    return BRepBuilderAPI_MakeEdge(circle, a0, a1).Edge()


def flank(centre, side):
    """The involute from the base circle (alpha = 0) to the tip circle on one side of a tooth;
    side = -1 for the right flank (angles below the centre), +1 for the left."""
    pts = HArray1_gp_Pnt(1, SAMPLES + 1)
    for i in range(SAMPLES + 1):
        a = ALPHA_TIP * i / SAMPLES
        r = RB / math.cos(a)
        pts.SetValue(i + 1, polar(r, centre + side * (HALF - (math.tan(a) - a))))
    interp = GeomAPI_Interpolate(pts, False, 1e-9)
    interp.Perform()
    if not interp.IsDone():
        raise Refused("GeomAPI_Interpolate could not fit the involute")
    return BRepBuilderAPI_MakeEdge(interp.Curve()).Edge()


def outline():
    """The gear's flat outline as a face in the plane z = 0."""
    wire = BRepBuilderAPI_MakeWire()
    pitch = 2 * math.pi / TEETH
    inv_tip = math.tan(ALPHA_TIP) - ALPHA_TIP
    for n in range(TEETH):
        c = n * pitch
        wire.Add(arc(RF, c - pitch + HALF, c - HALF))                 # the gap before this tooth
        wire.Add(line(polar(RF, c - HALF), polar(RB, c - HALF)))      # radial segment up
        wire.Add(flank(c, -1))                                        # right flank, base to tip
        wire.Add(arc(RA, c - (HALF - inv_tip), c + (HALF - inv_tip))) # tip
        f = flank(c, +1)                                              # left flank, base to tip,
        f.Reverse()                                                   # walked tip to base
        wire.Add(f)
        wire.Add(line(polar(RB, c + HALF), polar(RF, c + HALF)))      # radial segment down
    if not wire.IsDone():
        raise Refused("BRepBuilderAPI_MakeWire could not close the gear outline")
    return BRepBuilderAPI_MakeFace(wire.Wire(), True).Face()


def build():
    return BRepPrimAPI_MakePrism(outline(), gp_Vec(0, 0, HEIGHT)).Shape()
