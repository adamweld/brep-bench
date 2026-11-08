"""A V bit engraving a line that stops inside a block. A block, the box from (0, −10, −10) to (30,
10, 0), with a line engraved into its top by a V bit that stops inside it at both ends.

The bit is a solid cone whose axis is the vertical line through (5, 0), its point at (5, 0, −2), 2
below the block's top, opening upward at 90° (each side 45° from the axis) to a flat top of radius
5 at z = 3.

It slides without turning along the x axis by 15, so that its point ends at (20, 0, −2). Cut from
the block everything the bit passes through.

The bit's point lies on its axis, inside the block at both ends. The groove is a V 4 wide at the
top and 2 deep, along 15, its flanks planes at 45°, ended at each end by half of the cone."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone
from OCP.gp import gp, gp_Ax2, gp_Pnt

from sweep import cut, plane, slid

BLOCK = ((0, -10, -10), (30, 10, 0))
POINT = (5, 0, -2)  # the bit's point, on its upright axis
TOP = 3  # the height of its flat top
HALF_ANGLE = 45
SLIDE = (15, 0, 0)


def build():
    height = TOP - POINT[2]
    bit = BRepPrimAPI_MakeCone(gp_Ax2(gp_Pnt(*POINT), gp.DZ_s()), 0,
                               height * math.tan(math.radians(HALF_ANGLE)), height).Shape()
    # The cone's normal, (cos 45°)·r̂ − (sin 45°)·ẑ at the line of it along r̂, is square to the
    # slide (x) where r̂ = ±ŷ: its two lines in the plane x = 5 through its axis. The flat top
    # slides in its own plane.
    swept = slid(bit, SLIDE, [plane(POINT, (1, 0, 0))])
    return cut(BRepPrimAPI_MakeBox(gp_Pnt(*BLOCK[0]), gp_Pnt(*BLOCK[1])).Shape(), swept)
