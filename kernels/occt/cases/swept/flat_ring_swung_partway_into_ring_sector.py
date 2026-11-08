"""A ring lying flat, swung partway into a curved block. A curved block with a notch cut into it by
a ring lying flat, swung partway into it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The ring is a torus whose axis is the vertical line through (10, 0): the circle through its tube's
centers has radius 1 and lies in the plane z = 0, and its tube has radius 0.4.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the ring passes
through.

The ring's top and bottom circles stay in their planes and sweep the notch's flat roof and floor. A
circle about the z axis can meet the ring in two arcs, either side of its hole, whose sweeps run
into one another."""
import math

from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeFace
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.gp import gp, gp_Ax2, gp_Ax3, gp_Cylinder, gp_Pnt

from sweep import cut, plane, ring_sector, turn, turned

BLOCK = (8, 12, -1, 1, 0, 60)  # radii, heights and polar angles
CENTRE = (10, 0, 0)  # the ring's centre; its axis is the vertical line through it
RING, TUBE = 1, 0.4  # the radius of the circle through the tube's centres, and the tube's
START, STOP = -140, 30  # degrees about the z axis


def build():
    ring = BRepPrimAPI_MakeTorus(gp_Ax2(gp_Pnt(*CENTRE), gp.DZ_s()), RING, TUBE).Shape()
    # The turn's axis and the ring's are parallel. The ring's normal is square to the turn's
    # velocity where it lies in the plane y = 0 through both axes, and on its top and bottom
    # circles, where it is upright: the plane and the cylinder of radius 1 about the ring's axis
    # cut it along that contact.
    cylinder = BRepBuilderAPI_MakeFace(gp_Cylinder(gp_Ax3(gp_Pnt(*CENTRE), gp.DZ_s()), RING),
                                       0, 2 * math.pi, -2 * TUBE, 2 * TUBE).Face()
    tool = turn(ring, gp.OZ_s(), START)
    cuts = [turn(plane((0, 0, 0), (0, 1, 0)), gp.OZ_s(), START), turn(cylinder, gp.OZ_s(), START)]
    return cut(ring_sector(*BLOCK), turned(tool, gp.OZ_s(), STOP - START, cuts))
