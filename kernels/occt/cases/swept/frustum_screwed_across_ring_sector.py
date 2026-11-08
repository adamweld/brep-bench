"""A helical groove milled by a tapered cutter. A curved block with a helical groove milled through
it by a tapered cutter screwed across it.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The cutter is a frustum of a cone about the vertical line through (10, 0): radius 0.4 at z = −0.6,
widening to radius 1 at z = 1.4, its ends flat. It stands out through the block's top.

It is carried along a screw about the z axis: it turns right-handed about it (from +x toward +y)
and rises 2 for every whole turn, rigidly, from a turn of −140° to one of +140°. It starts clear of
the block, passes wholly through it, and ends clear of it. Cut from the block everything the cutter
passes through."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from sweep import cut, plane, ring_sector, screw, screwed

BLOCK = (8, 12, -1, 1, 0, 60)  # radii, heights and polar angles
AXIS = 10  # the cutter's axis is the vertical line through (10, 0)
LOW, HIGH = -0.6, 1.4
LOW_RADIUS, HIGH_RADIUS = 0.4, 1
LEAD = 2  # rise per whole turn
START, STOP = -140, 140  # degrees


def build():
    frustum = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(AXIS, 0, LOW), gp.DZ_s()), LOW_RADIUS,
                                   HIGH_RADIUS, HIGH - LOW).Shape()
    tool = screw(frustum, LEAD, START)
    # The cone widens upward at α, tan α = 0.6/2, from its apex A = (10, 0, −0.6 − 0.4/tan α).
    # At its line at angle θ about its axis, its normal is cos α·r̂(θ) − sin α·ẑ, and a point p
    # there moves with velocity ẑ × p + kẑ, k = 2/2π: the two are square where
    # 10 cos α sin θ = k sin α, sin θ = k tan α/10, on two lines through A symmetric about the
    # plane x = 10. Those two lines span the plane through A containing x̂ and the line's
    # direction (0, sin α sin θ, cos α), whose normal is (0, −cos α, sin α sin θ): it cuts the
    # cone along its contact. Its flat ends rise or sink all over.
    alpha = math.atan((HIGH_RADIUS - LOW_RADIUS) / (HIGH - LOW))
    apex = (AXIS, 0, LOW - LOW_RADIUS / math.tan(alpha))
    sin_theta = LEAD / (2 * math.pi) * math.tan(alpha) / AXIS
    contact = screw(plane(apex, (0, -math.cos(alpha), math.sin(alpha) * sin_theta)), LEAD, START)
    return cut(ring_sector(*BLOCK), screwed(tool, LEAD, START, STOP, [contact]))
