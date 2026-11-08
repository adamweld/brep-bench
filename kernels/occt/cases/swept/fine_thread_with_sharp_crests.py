"""A thread so fine its crests are sharp. A rod, the cylinder of radius 5 about the z axis from z =
0 to z = 20, threaded by a bar carried along a screw so fine that the turns of the groove meet: the
rod's own surface is cut away everywhere, and what is left between neighbouring turns is a thread
whose crest is a sharp helical edge.

The bar, as placed, is the box 4.5 ≤ x ≤ 12, −8 ≤ y ≤ 8, −0.45 ≤ z ≤ 0.45: it points at the rod's
axis, its flat tip (the face x = 4.5) 0.5 inside the rod, and it is 0.9 wide along the axis.

The screw is about the z axis: the bar turns right-handed about it (from +x toward +y) and rises 1
for every whole turn, rigidly. It runs from a turn of −180° (the bar lowered 0.5, wholly below the
rod) to a turn of 7380° (raised 20.5, wholly above it). Cut from the rod everything the bar passes
through.

At distance ρ from the axis the groove is 0.9 + acos(4.5/ρ)/π wide along it (its walls are where
the tip's long edges pass). That reaches the whole lead, 1, at ρ = 4.5/cos 18° ≈ 4.7316: there the
walls of neighbouring turns meet in the crest, and beyond it nothing of the rod is left. Each turn
of the thread is about 0.23 deep, 0.1 wide at its root and sharp at its crest."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Common
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from bench import Refused
from sweep import cut, flat_contacts_screwed, screw, screwed

ROD_RADIUS, ROD_LENGTH = 5, 20
BAR = ((4.5, -8, -0.45), (12, 8, 0.45))
LEAD = 1  # rise per whole turn
START, STOP = -180, 7380  # degrees
# A screw about the rod's axis keeps every point's distance from it, so only the part of the bar
# within the rod's radius ever enters the rod; the bar is cut down to the part within this.
REACH = 5.01


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
    # Swept 270° at a time: a span overlaps itself only once it comes round to within the
    # bar's own width of where it began, past 360° − 2 acos(4.5/5.01) ≈ 308°.
    # Every circle about the axis meets the bar, cut down to within REACH of it, in one arc, so
    # no point is entered twice in a pass, and the leading faces of each span are swept whole.
    pieces = screwed(tool, LEAD, START, STOP, flat_contacts_screwed(tool, LEAD), span=270,
                     whole=True)
    rod = BRepPrimAPI_MakeCylinder(gp.XOY_s(), ROD_RADIUS, ROD_LENGTH).Shape()
    return cut(rod, pieces)
