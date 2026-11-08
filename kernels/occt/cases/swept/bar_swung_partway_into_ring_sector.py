"""A small bar swung partway into a curved block. A curved block with a notch cut into it by a
small bar swung partway into it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The bar is the box 10 ≤ x ≤ 11, −0.5 ≤ y ≤ 0.5, −0.6 ≤ z ≤ 0.6.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the bar passes
through.

The notch's far wall is the cylinder of radius √(11² + 0.5²) that the bar's outer corners sweep,
its near wall the cylinder of radius 10 its inner face sweeps, and it ends in the bar's own faces
where it stops."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Pnt

from sweep import cut, flat_contacts_turned, ring_sector, turn, turned

BLOCK = (8, 12, -1, 1, 0, 60)  # radii, heights and polar angles
BAR = ((10, -0.5, -0.6), (11, 0.5, 0.6))
START, STOP = -140, 30  # degrees about the z axis


def build():
    bar = turn(BRepPrimAPI_MakeBox(gp_Pnt(*BAR[0]), gp_Pnt(*BAR[1])).Shape(), gp.OZ_s(), START)
    # Each flat face meets the turn square-on along the plane through the axis and its normal:
    # the faces x = 10 and x = 11 along their middles, y = 0; the others nowhere, or, level, not
    # at all.
    swept = turned(bar, gp.OZ_s(), STOP - START, flat_contacts_turned(bar, gp.OZ_s()))
    return cut(ring_sector(*BLOCK), swept)
