"""A drum plunged aslant into a plate, stopping inside. A plate, the box from (0, −5, 0) to (20, 5,
4), with a pocket cut into its underside by a drum pushed up aslant into it, stopping inside.

The drum is a solid cylinder of radius 2 lying on its side: its axis is the line y = 0, z = −5, and
it runs from x = 6 to x = 14, its ends flat.

It slides without turning by 8 along the direction (0.3, 0, √(1 − 0.3²)), leaning 0.3 toward +x
from straight up, so that its axis ends at height −5 + 8√0.91 ≈ 2.63, inside the plate. Cut from
the plate everything the drum passes through.

Pushed aslant, the drum's flat ends lean to the push: each rim sweeps an oblique elliptic cylinder,
the side's lines at y = ±2 sweep the planes tangent along them, and the pocket ends in the drum's
own side where it stops."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCylinder
from OCP.gp import gp, gp_Ax2, gp_Pnt

from sweep import cut, plane, slid

PLATE = ((0, -5, 0), (20, 5, 4))
DRUM_START = (6, 0, -5)  # the middle of the drum's end at x = 6, on its axis
DRUM_RADIUS, DRUM_LENGTH = 2, 8
LEAN = 0.3  # the push's x component, of a unit direction
PUSH = 8


def build():
    drum = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(*DRUM_START), gp.DX_s()), DRUM_RADIUS,
                                    DRUM_LENGTH).Shape()
    push = (PUSH * LEAN, 0, PUSH * math.sqrt(1 - LEAN ** 2))
    # The drum's side, at (x, 2 cos t, −5 + 2 sin t), has normal (0, cos t, sin t), square to the
    # push where sin t = 0: its two lines at y = ±2, where the plane z = −5 through its axis cuts
    # it. Its flat ends lean to the push and lead or trail all over.
    swept = slid(drum, push, [plane(DRUM_START, (0, 0, 1))])
    return cut(BRepPrimAPI_MakeBox(gp_Pnt(*PLATE[0]), gp_Pnt(*PLATE[1])).Shape(), swept)
