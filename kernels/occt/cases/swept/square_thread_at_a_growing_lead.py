"""A rod threaded at a lead that grows along it. A rod, the cylinder of radius 5 about the z axis
from z = 0 to z = 20, threaded by a bar carried along a path whose lead grows as it climbs, as a
variable-pitch screw is cut.

The bar, as placed, is the box 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1: it points at the rod's axis, its
flat tip 1 inside the rod, and it is 2 wide along the axis.

When it has turned by θ about the z axis, right-handed (from +x toward +y), it has risen 6s +
0.3s², s = θ/360°, rigidly. θ runs from −90° (risen −1.48125, the bar wholly below the rod) to
1260° (risen 24.675, wholly above it). Cut from the rod everything the bar passes through.

The lead grows from about 6 at the bottom to about 8 at the top: no screw carries the bar, and the
groove's section is different at every height."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from sweep import cut, flat_contacts_screwed, moved, threaded, threaded_motion

ROD_RADIUS, ROD_LENGTH = 5, 20
BAR = ((4, -8, -1), (12, 8, 1))
LEAD, GROWTH = 6, 0.3  # it rises 6s + 0.3s² at s whole turns
START, STOP = -90, 1260  # degrees
# The motion turns about the rod's axis and rises along it, keeping every point's distance from
# it, so only the part of the bar within the rod's radius ever enters the rod; the bar is cut
# down to the part within this.
REACH = 6


def build():
    bar = BRepPrimAPI_MakeBox(gp_Pnt(*BAR[0]), gp_Pnt(*BAR[1])).Shape()
    within = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -10), gp.DZ_s()), REACH, 20).Shape()
    near = BRepAlgoAPI_Common(bar, within)
    if not near.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the bar's part near the rod")
    tool = moved(near.Shape(), threaded_motion(LEAD, GROWTH, 0, START))
    # A point p moves with velocity ẑ × p + kẑ, k the rise per radian, which grows. The tip's
    # normal is level, so it meets the motion square-on where it meets the turn, along its
    # middle, through the axis, whatever k; the level faces rise all the while (k > 0 from the
    # start) or sink, and lead or trail all over. So the same faces lead all the way.
    # Every circle about the axis meets the bar, cut down to within REACH of it, in one arc, so
    # no point is entered twice in a pass, and the leading faces of each span are swept whole.
    pieces = threaded(tool, LEAD, START, STOP, flat_contacts_screwed(tool, LEAD), growth=GROWTH,
                      whole=True)
    rod = BRepPrimAPI_MakeCylinder(gp.XOY_s(), ROD_RADIUS, ROD_LENGTH).Shape()
    return cut(rod, pieces)
