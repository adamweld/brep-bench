"""A thin plate with a half-round pocket in its side, the pocket's rim and its two upright edges
rounded together. The box from the origin to (30, 20, 2), a pocket of radius 5 about the line
x = 0, y = 10 down to z = 1/2. The semicircle where the wall meets the top to radius 1/2, the two
upright edges where the wall meets the side x = 0 to radius 1, each round running the whole length
of its edge so that at the top corners they meet in a crease. Built as one fillet of all the edges, each with
its own radius. Finding: one fillet of the edges closes the corners as the words say (1139.795 against 1139.793 from the set); the rim round first and the upright edges after is refused, the upright edges first leaves 1139.26."""
import math

from blend import edges_where, rounded_together
from edit import box, cylinder, less


def build():
    plate = less(box((0, 0, 0), (30, 20, 2)), cylinder((0, 10, 0.5), 5, 2))
    uprights = edges_where(plate, lambda p: abs(p.X()) < 1e-7 and abs(p.Z() - 1.25) < 1e-7
                           and (abs(p.Y() - 5) < 1e-7 or abs(p.Y() - 15) < 1e-7))
    assert len(uprights) == 2, len(uprights)
    rim = edges_where(plate, lambda p: abs(p.Z() - 2) < 1e-7 and abs(math.hypot(p.X(), p.Y() - 10) - 5) < 1e-7)
    assert len(rim) == 2, len(rim)  # the semicircle, split in two at the cylinder's seam
    return rounded_together(plate, [(1, uprights), (0.5, rim)])
