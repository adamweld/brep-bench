"""A thread with a run-out: a bar screwed up a rod, then withdrawn. A rod, the cylinder of radius 5
about the z axis from z = 0 to z = 20, with a thread cut partway up it by a bar that is then
withdrawn straight out of its groove, as a lathe's tool runs out of a thread. The tool's path is
made of two pieces.

The bar, as placed, is the box 4 ≤ x ≤ 12, −8 ≤ y ≤ 8, −1 ≤ z ≤ 1: it points at the rod's axis, its
flat tip 1 inside the rod, and it is 2 wide along the axis.

First it is carried along a screw about the z axis, turning right-handed (from +x toward +y) and
rising 6 for every whole turn, rigidly, from a turn of −90° (lowered 1.5, wholly below the rod) to
one of +900° (raised 15), where it points along −x, its tip at (−4, 0, 15). Then it slides without
turning along −x by 3, straight back along its own length, until its tip is at (−7, 0, 15), clear
of the rod. Cut from the rod everything the bar passes through.

The two pieces share the bar where the screw stops and the slide starts. The run-out is the bar's
section carried straight out of the groove."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from sweep import cut, flat_contacts_screwed, screw, screwed, slid

ROD_RADIUS, ROD_LENGTH = 5, 20
BAR = ((4, -8, -1), (12, 8, 1))
LEAD = 6  # rise per whole turn
START, STOP = -90, 900  # degrees
RUN_OUT = (-3, 0, 0)  # then it slides straight back along its length
# Neither piece of the path brings any point of the bar nearer the rod's axis: the screw keeps
# every point's distance from it, and the slide carries the bar, pointing along −x, further out.
# So only the part of the bar within the rod's radius ever enters the rod; the bar is cut down
# to the part within this.
REACH = 6


def build():
    bar = BRepPrimAPI_MakeBox(gp_Pnt(*BAR[0]), gp_Pnt(*BAR[1])).Shape()
    within = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -10), gp.DZ_s()), REACH, 20).Shape()
    near = BRepAlgoAPI_Common(bar, within)
    if not near.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the bar's part near the rod")
    tool = screw(near.Shape(), LEAD, START)
    # The screw: the tip cut along its middle, through the axis, where it meets the motion
    # square-on; the level faces rise or sink all over.
    # Every circle about the axis meets the bar, cut down to within REACH of it, in one arc, so
    # no point is entered twice in a pass, and the leading faces of each span are swept whole.
    pieces = screwed(tool, LEAD, START, STOP, flat_contacts_screwed(tool, LEAD), whole=True)
    # The slide: the bar, pointing along −x, slides along its length. Its tip trails, its flat
    # sides slide in their own planes, and its far face, the cylinder of radius 6, leads all
    # over (its contact with a slide along x would be on the plane x = 0, far from it).
    pieces += slid(screw(near.Shape(), LEAD, STOP), RUN_OUT)
    rod = BRepPrimAPI_MakeCylinder(gp.XOY_s(), ROD_RADIUS, ROD_LENGTH).Shape()
    return cut(rod, pieces)
