"""A curved block cut by a bar's side facing the axis squarely. A curved block, cut by the inner
side of a bar turned past it about the block's own axis.

The block is a sector of a ring about the z axis: everything with distance from the z axis between
8 and 12, with z between −1 and 1, and with polar angle between 0° and 60° (measured from +x toward
+y).

The bar is a box leaning out from upright in the plane y = 0. Its inner side is the plane through
(10, 0, 0) square to the direction (cos 20°, 0, −sin 20°), so it faces the z axis squarely and
leans out 20° from upright: at height z it stands 10 + z·tan 20° from the axis. The bar is
everything from that plane to 3 beyond it along (cos 20°, 0, −sin 20°), within 2 of the plane y =
0, and within 3 of the plane through (10, 0, 0) square to (sin 20°, 0, cos 20°) (its length runs
along that direction). So it stands past the block above and below, and its outer side lies beyond
the block's outer wall.

The bar turns rigidly about the z axis, right-handed (from +x toward +y), from −140° to +140°: it
starts clear of the block, passes through it, and ends clear of it. Cut from the block everything
the bar passes through.

Every line of the inner side that runs along the bar's length is in contact all along it or not at
all; the one in the plane through the axis cuts the block's new wall, a cone. What is left of the
block is the part of it within 10 + z·tan 20° of the axis."""
import math

from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp, gp_Ax2, gp_Dir, gp_Pnt

from sweep import cut, flat_contacts_turned, ring_sector, turn, turned

BLOCK = (8, 12, -1, 1, 0, 60)  # radii, heights and polar angles
LEAN = math.radians(20)
INNER = (10, 0, 0)  # the middle of the bar's inner side
THICK, HALF_WIDTH, HALF_LENGTH = 3, 2, 3  # along u, y and the bar's length
START, STOP = -140, 140  # degrees about the z axis


def build():
    u = (math.cos(LEAN), 0, -math.sin(LEAN))  # out from the inner side
    along = (math.sin(LEAN), 0, math.cos(LEAN))  # the bar's length; u × ŷ
    corner = [c - HALF_WIDTH * y - HALF_LENGTH * a for c, y, a in zip(INNER, (0, 1, 0), along)]
    frame = gp_Ax2(gp_Pnt(*corner), gp_Dir(*along), gp_Dir(*u))
    bar = BRepPrimAPI_MakeBox(frame, THICK, 2 * HALF_WIDTH, 2 * HALF_LENGTH).Shape()
    bar = turn(bar, gp.OZ_s(), START)
    # Each flat face meets the turn square-on along the plane through the axis and its normal:
    # for the inner and outer sides and the ends, all with normals in the plane y = 0, that plane
    # itself; the sides y = ±2 nowhere.
    swept = turned(bar, gp.OZ_s(), STOP - START, flat_contacts_turned(bar, gp.OZ_s()))
    return cut(ring_sector(*BLOCK), swept)
