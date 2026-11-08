"""A half drum swung partway into a curved block, its arcs ending at its cut. A curved block with a
notch cut into it by half a drum swung partway into it, flat face first.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The tool is half of the solid cylinder of radius 1 about the vertical line through (10, 0), from z
= −0.5 to z = 0.5: the half with y ≤ 0, cut off by the plane y = 0 through its axis. Its flat face
is the rectangle 9 ≤ x ≤ 11, y = 0, |z| ≤ 0.5, and its round side faces −y.

It turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +30°: it starts
clear of the block and stops halfway across it. Cut from the block everything the half drum passes
through.

The half drum leads with its flat face, so the notch is the ring between radii 9 and 11 and heights
±0.5 from the block's edge at 0° to the flat face where it stops, at 30°. Its round side's arcs end
at that face, at the two corners whose paths are the notch's inner and outer edges."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from sweep import cut, ring_sector, turn, turned

BLOCK = (8, 12, -1, 1, 0, 60)  # radii, heights and polar angles
AXIS = 10  # the drum's axis is the vertical line through (10, 0)
RADIUS, LOW, HIGH = 1, -0.5, 0.5
START, STOP = -140, 30  # degrees about the z axis


def build():
    # half a turn of the drum's section, from −x round through −y to +x: the half with y ≤ 0
    half = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(AXIS, 0, LOW), gp.DZ_s(), gp.DX_s().Reversed()),
                                    RADIUS, HIGH - LOW, math.pi).Shape()
    tool = turn(half, gp.OZ_s(), START)
    # Its round side meets the turn square-on only along its two edges in the plane y = 0
    # through both axes, and trails everywhere between; its flat face, in a plane through the
    # turn's axis, leads all over; its ends turn in their own planes. No face needs splitting.
    swept = turned(tool, gp.OZ_s(), STOP - START)
    return cut(ring_sector(*BLOCK), swept)
