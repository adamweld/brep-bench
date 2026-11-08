"""A rod threaded by a short bar whose corners cut. A rod, the cylinder of radius 5 about the z
axis from z = 0 to z = 20, with a thread cut along it by a short bar carried along a screw, so that
the bar's corners, and not only its edges, carry the groove's walls.

The bar, as placed, is the box 4 ≤ x ≤ 12, −1.5 ≤ y ≤ 1.5, −1 ≤ z ≤ 1: it points at the rod's axis,
its flat tip (the face x = 4) 1 inside the rod, 2 wide along the axis and 3 across.

The screw is about the z axis: the bar turns right-handed about it (from +x toward +y) and rises 6
for every whole turn, rigidly. It runs from a turn of −90° (the bar lowered 1.5, wholly below the
rod) to a turn of 1350° (raised 22.5, wholly above it). Cut from the rod everything the bar passes
through.

The bar's four corners (4, ±1.5, ±1) lie inside the rod. Out to distance √(4² + 1.5²) ≈ 4.272 from
the axis the groove's walls are carried by the tip's long edges, as a long bar's are; beyond it, by
the edges where the bar's sides y = ±1.5 meet its faces z = ±1. The two meet at the corners'
paths."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from sweep import cut, flat_contacts_screwed, screw, screwed

ROD_RADIUS, ROD_LENGTH = 5, 20
BAR = ((4, -1.5, -1), (12, 1.5, 1))
LEAD = 6  # rise per whole turn
START, STOP = -90, 1350  # degrees
# A screw about the rod's axis keeps every point's distance from it, so only the part of the bar
# within the rod's radius ever enters the rod; the bar is cut down to the part within this.
REACH = 6


def build():
    bar = BRepPrimAPI_MakeBox(gp_Pnt(*BAR[0]), gp_Pnt(*BAR[1])).Shape()
    within = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -10), gp.DZ_s()), REACH, 20).Shape()
    near = BRepAlgoAPI_Common(bar, within)
    if not near.IsDone():
        raise Refused("BRepAlgoAPI_Common declined the bar's part near the rod")
    tool = screw(near.Shape(), LEAD, START)
    # Of the faces left, the cylinder of radius 6 is carried along itself; the flat ones are
    # cut along their contact: the tip along its middle, through the axis, the sides square to
    # it nowhere; the level faces rise or sink all over.
    # Every circle about the axis meets the bar, cut down to within REACH of it, in one arc, so
    # no point is entered twice in a pass, and the leading faces of each span are swept whole.
    pieces = screwed(tool, LEAD, START, STOP, flat_contacts_screwed(tool, LEAD), whole=True)
    rod = BRepPrimAPI_MakeCylinder(gp.XOY_s(), ROD_RADIUS, ROD_LENGTH).Shape()
    return cut(rod, pieces)
