"""A closed ring whose section turns from a square to a circle and back. Centreline: the circle of
radius 20 about the z axis. At angle theta the section is the radial blend, at parameter t,
between the square of half-width 3 and the circle of radius 3, t running 0 -> 1 over the first
half turn and 1 -> 0 over the second. Built as two lofts through sections every 10 degrees,
each section a periodic spline through 96 points of the blend, and fused."""
import math

from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.GeomAPI import GeomAPI_Interpolate
from OCP.collections import HArray1_gp_Pnt
from OCP.gp import gp_Pnt

from bench import Refused

RADIUS, HALF = 20.0, 3.0
POINTS = 96
STEP = 10  # degrees between sections


def blend(alpha, t):
    square = HALF / max(abs(math.cos(alpha)), abs(math.sin(alpha)))
    return (1 - t) * square + t * HALF


def section(theta_deg):
    theta = math.radians(theta_deg)
    t = theta_deg / 180 if theta_deg <= 180 else 2 - theta_deg / 180
    cx, cy = math.cos(theta), math.sin(theta)
    pts = HArray1_gp_Pnt(1, POINTS)
    for i in range(POINTS):
        a = 2 * math.pi * i / POINTS
        u, z = blend(a, t) * math.cos(a), blend(a, t) * math.sin(a)
        pts.SetValue(i + 1, gp_Pnt((RADIUS + u) * cx, (RADIUS + u) * cy, z))
    interp = GeomAPI_Interpolate(pts, True, 1e-9)
    interp.Perform()
    edge = BRepBuilderAPI_MakeEdge(interp.Curve()).Edge()
    return BRepBuilderAPI_MakeWire(edge).Wire()


def half(start):
    loft = BRepOffsetAPI_ThruSections(True, False)
    for k in range(180 // STEP + 1):
        loft.AddWire(section(start + k * STEP))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections failed on a half of the ring")
    return loft.Shape()


def build():
    fuse = BRepAlgoAPI_Fuse(half(0), half(180))
    if not fuse.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the two halves")
    fuse.SimplifyResult()
    return fuse.Shape()
