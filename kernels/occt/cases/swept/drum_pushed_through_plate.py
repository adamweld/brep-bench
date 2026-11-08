"""A slot punched by a drum pushed edge first. A plate, the box from (0, −5, 0) to (20, 5, 4), with
a slot punched through it by a drum.

The drum is a solid cylinder of radius 2 lying on its side: its axis is the line y = 0, z = −5, and
it runs from x = 6 to x = 14, its ends flat.

It slides without turning straight up by 16, from wholly below the plate to wholly above it. Cut
from the plate everything the drum passes through.

The drum touches what it sweeps along a closed curve: along its side's lowest and highest lines
nowhere (they move along themselves), along the two lines of its side at y = ±2, and across both
flat ends, which the push keeps in their own planes. The slot is its shadow along the push, the
rectangle 6 ≤ x ≤ 14, |y| ≤ 2, through the plate."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from sweep import cut, plane, slid

PLATE = ((0, -5, 0), (20, 5, 4))
DRUM_START = (6, 0, -5)  # the middle of the drum's end at x = 6, on its axis
DRUM_RADIUS, DRUM_LENGTH = 2, 8
PUSH = (0, 0, 16)


def build():
    drum = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*DRUM_START), gp.DX_s()), DRUM_RADIUS,
                                    DRUM_LENGTH).Shape()
    # The drum's side is square to the push along its two lines at y = ±2, where its normal is
    # level: the plane z = −5 through its axis cuts it there. Its flat ends slide in their own
    # planes.
    swept = slid(drum, PUSH, [plane(DRUM_START, (0, 0, 1))])
    return cut(BRepPrimAPI_MakeBox(gp_Pnt(*PLATE[0]), gp_Pnt(*PLATE[1])).Shape(), swept)
