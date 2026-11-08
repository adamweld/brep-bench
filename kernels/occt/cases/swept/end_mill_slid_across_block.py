"""A slot cut by an end mill slid across a block. A block, the box from (0, −5, 0) to (20, 5, 10),
with a slot cut across its top by an end mill.

The end mill is a solid cylinder of radius 2 standing upright about the vertical line through (−5,
0), from z = 7 up to z = 15: its flat end is 3 below the block's top.

It slides without turning along the x axis by 30, from x = −5 to x = 25, from clear before the
block to clear past it. Cut from the block everything the mill passes through.

Its flat end, which the slide keeps in its own plane, touches all over and sweeps the slot's floor;
the two lines of its side square to the slide sweep the walls, meeting the floor in sharp edges.
The slot is |y| ≤ 2, z ≥ 7, along the block's whole length."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from sweep import cut, plane, slid

BLOCK = ((0, -5, 0), (20, 5, 10))
MILL_FOOT = (-5, 0, 7)  # the middle of the mill's flat end
RADIUS, HEIGHT = 2, 8
SLIDE = (30, 0, 0)


def build():
    mill = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*MILL_FOOT), gp.DZ_s()), RADIUS, HEIGHT).Shape()
    # The mill's side is square to the slide along its two lines at y = ±2, where its normal is
    # along ±y: the plane x = −5 through its axis cuts it there. Its flat ends slide in their
    # own planes.
    swept = slid(mill, SLIDE, [plane(MILL_FOOT, (1, 0, 0))])
    return cut(BRepPrimAPI_MakeBox(gp_Pnt(*BLOCK[0]), gp_Pnt(*BLOCK[1])).Shape(), swept)
