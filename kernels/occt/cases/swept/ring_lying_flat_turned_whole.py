"""A ring lying flat, turned a whole turn about a line beside it. The solid a ring fills as it is
turned a whole turn about an upright line beside it.

The ring is a torus whose axis is the vertical line through (8, 0): the circle through its tube's
centers has radius 3, and its tube has radius 1, centered on the plane z = 0.

It turns rigidly a whole turn about the z axis. Make the solid of everything the ring passes
through.

The ring's top and bottom circles stay in their planes and sweep flat annuli; its circles nearest
and farthest from the z axis at each height sweep the inner and outer walls. The result is a solid
ring about the z axis, from 4 to 12 out, its section in each half-plane through the z axis a
stadium: the band |z| ≤ 1 between 5 and 11 out, ended by half discs of radius 1."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Ax3, gp_Cylinder, gp_Pnt

from sweep import plane, turned, unified, united

CENTRE = (8, 0, 0)  # the ring's centre; its axis is the vertical line through it
RING, TUBE = 3, 1  # the radius of the circle through the tube's centres, and the tube's


def build():
    ring = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(*CENTRE), gp.DZ_s()), RING, TUBE).Shape()
    # The turn's axis and the ring's are parallel. The ring's normal is square to the turn's
    # velocity where it lies in the plane y = 0 through both axes, and on its top and bottom
    # circles, where it is upright: the plane and the cylinder of radius 3 about the ring's axis
    # cut it along that contact.
    cylinder = BRepBuilderAPI_MakeFace(gp_Cylinder(gp_Ax3(gp_Pnt(*CENTRE), gp.DZ_s()), RING),
                                       0, 2 * math.pi, -2 * TUBE, 2 * TUBE).Face()
    _, *swept = turned(ring, gp.OZ_s(), 360, [plane((0, 0, 0), (0, 1, 0)), cylinder])
    # Turned a whole turn, the ring is swept round back to where it started; and no point stays
    # inside it all the way round (half a turn takes it clear of where it was), so each point
    # it covers where it starts it covers again later, coming in across a leading face. What the
    # leading faces sweep is the whole solid.
    return unified(united(swept))
