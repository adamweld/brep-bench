"""A thin plate with a half-round pocket in its side, the pocket's rim rounded. The box (0, 0, 0)
to (30, 20, 2) less the cylinder of radius 5 about x = 0, y = 10 from z = 1/2 up; the semicircle
where the pocket's wall meets the top rounded to radius 1/2."""
import math

from blend import edges_where, rounded
from edit import box, cylinder, less


def build():
    plate = less(box((0, 0, 0), (30, 20, 2)), cylinder((0, 10, 0.5), 5, 2))
    rim = edges_where(plate, lambda p: abs(p.Z() - 2) < 1e-7
                      and abs(math.hypot(p.X(), p.Y() - 10) - 5) < 1e-7)
    assert len(rim) == 2, len(rim)  # the semicircle, split in two at the cylinder's seam
    return rounded(plate, 0.5, rim, "the pocket's rim")
