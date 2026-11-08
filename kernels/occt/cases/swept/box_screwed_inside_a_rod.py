"""A sealed helical tunnel: a box screwed inside a rod. A rod, the cylinder of radius 5 about the z
axis from z = 0 to z = 20, with a sealed helical tunnel inside it: everything a small box passes
through as it is carried along a screw wholly within the rod.

The box, as placed, is 2.5 ≤ x ≤ 3.5, −0.5 ≤ y ≤ 0.5, 2 ≤ z ≤ 3.

The screw is about the z axis: the box turns right-handed about it (from +x toward +y) and rises 4
for every whole turn, rigidly, from a turn of 0° to one of 720°, ending as the box 2.5 ≤ x ≤ 3.5,
−0.5 ≤ y ≤ 0.5, 10 ≤ z ≤ 11. Cut from the rod everything the box passes through.

The box stays inside the rod all the way, so the result is the rod with a closed helical cavity two
turns long, square in section and ended by the box's own faces: two shells. The box's level faces
sweep helicoidal sheets and its upright faces cylinders and ruled sheets, meeting along its edges'
helices."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Pnt

from sweep import cut, flat_contacts_screwed, screwed

ROD_RADIUS, ROD_LENGTH = 5, 20
BOX = ((2.5, -0.5, 2), (3.5, 0.5, 3))
LEAD = 4  # rise per whole turn
START, STOP = 0, 720  # degrees


def build():
    box = BRepPrimAPI_MakeBox(gp_Pnt(*BOX[0]), gp_Pnt(*BOX[1])).Shape()
    # Each flat face is cut along its contact: the faces x = 2.5 and x = 3.5 along their middles,
    # y = 0, through the axis; the faces y = ±0.5 square to it nowhere on them; the level faces
    # rise or sink all over.
    pieces = screwed(box, LEAD, START, STOP, flat_contacts_screwed(box, LEAD))
    return cut(BRepPrimAPI_MakeCylinder(gp.XOY_s(), ROD_RADIUS, ROD_LENGTH).Shape(), pieces)
