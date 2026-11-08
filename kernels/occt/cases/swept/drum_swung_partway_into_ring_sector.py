"""A drum swung partway into a curved block. A curved block with a notch cut into it by a drum
swung partway into it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The drum is a solid cylinder of radius 1 about the vertical line through (10, 0), from z = −0.5 to
z = 0.5, its ends flat.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the drum passes
through.

The notch runs in from the block's flat side at 0°, between the cylinders of radii 9 and 11 about
the z axis and the planes z = ±0.5, and ends in the half of the drum's side that leads where it
stops."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from sweep import cut, plane, ring_sector, turn, turned

BLOCK = (8, 12, -1, 1, 0, 60)  # radii, heights and polar angles
DRUM_AXIS = (10, 0)  # the drum stands on the vertical line through this point
DRUM_RADIUS, DRUM_LOW, DRUM_HIGH = 1, -0.5, 0.5
START, STOP = -140, 30  # degrees about the z axis


def build():
    drum = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*DRUM_AXIS, DRUM_LOW), gp.DZ_s()), DRUM_RADIUS,
                                    DRUM_HIGH - DRUM_LOW).Shape()
    drum = turn(drum, gp.OZ_s(), START)
    # The turn's axis and the drum's are parallel, so the plane through both is square to the
    # turn's velocity all along it: the drum's side meets the motion square-on, its normal along
    # the velocity's own line, nowhere but there, and the two lines where that plane cuts the
    # side are its contact. Its flat ends turn in their own planes. (Point (10, 0) turned with the
    # drum to where it starts.)
    through_axes = turn(plane((0, 0, 0), (0, 1, 0)), gp.OZ_s(), START)
    swept = turned(drum, gp.OZ_s(), STOP - START, [through_axes])
    return cut(ring_sector(*BLOCK), swept)
