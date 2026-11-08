"""A ring lying flat, slid across farther than its hole. The solid a ring fills as it slides
across, square to its own axis, farther than the width of its hole.

The ring is a torus about the z axis centered on the origin: the circle through its tube's centers
has radius 6, and its tube has radius 2.

It slides without turning along the x axis by 15, ending centered on (15, 0, 0). Make the solid of
everything the ring passes through.

The torus's top and bottom circles, of radius 6 at z = ±2, stay in their planes as it slides, so
each sweeps a flat face: the region within 6 of the segment from (0, 0) to (15, 0). The slide is
longer than the hole is wide, so the hole is filled; the inner half of the torus touches nowhere."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax3, gp_Cylinder

from sweep import plane, slid, united

RING, TUBE = 6, 2  # the radius of the circle through the tube's centres, and the tube's
SLIDE = (15, 0, 0)


def build():
    ring = BRepPrimAPI_MakeTorus(gp.XOY_s(), RING, TUBE).Shape()
    # At the point of the tube's circle turned u about the z axis and v round the tube, the
    # torus's normal is cos v·(cos u, sin u, 0) + sin v·ẑ, square to the slide (x) where
    # cos v·cos u = 0: on its two circles in the plane x = 0 through its axis, and on its top
    # and bottom circles, of radius 6 at z = ±2, where the cylinder of radius 6 cuts it.
    cylinder = BRepBuilderAPI_MakeFace(gp_Cylinder(gp_Ax3(gp.XOY_s()), RING),
                                       0, 2 * math.pi, -2 * TUBE, 2 * TUBE).Face()
    return united(slid(ring, SLIDE, [plane((0, 0, 0), (1, 0, 0)), cylinder]))
