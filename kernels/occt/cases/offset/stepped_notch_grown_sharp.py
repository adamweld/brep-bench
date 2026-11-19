"""A plate with a stepped notch grown sharp until its narrow part closes. A plate whose outline in
the xy plane is the rectangle from the origin to (30, 12) with a stepped notch cut into its top
edge: the polygon through (0, 0), (30, 0), (30, 12), (18, 12), (18, 8), (16, 8), (16, 4), (14, 4),
(14, 8), (12, 8), (12, 12) and (0, 12), extruded from z = 0 to z = 10. The notch is 6 wide for its
upper 4 (12 ≤ x ≤ 18, 8 ≤ y ≤ 12) and 2 wide for its lower 4 (14 ≤ x ≤ 16, 4 ≤ y ≤ 8).

Grow it by 1.5 with its edges kept sharp: move each face 1.5 outward along its normal, and extend
the moved faces until they meet, so that no round is added and every edge stays where its two faces
meet. Where moved faces cross, the solid is all they enclose. The narrow part's two walls, 2 apart,
each move 1.5 toward the other and cross: it closes, and its floor, moved up to y = 5.5, lies inside
the solid, no face of it. The wide part's walls move to x = 13.5 and x = 16.5, and its floor, the
two steps at y = 8 on either side of the narrow part, moves up to y = 9.5.

So the grown plate is the rectangle from (−1.5, −1.5) to (31.5, 13.5) with the notch
13.5 ≤ x ≤ 16.5, y ≥ 9.5 left in its top, extruded from z = −1.5 to z = 11.5."""
from OCP.GeomAbs import GeomAbs_Intersection

from edit import extruded
from offset import offset

OUTLINE = [(0, 0), (30, 0), (30, 12), (18, 12), (18, 8), (16, 8), (16, 4), (14, 4), (14, 8),
           (12, 8), (12, 12), (0, 12)]
THICKNESS = 10
DISTANCE = 1.5


def build():
    plate = extruded([(x, y, 0) for x, y in OUTLINE], (0, 0, THICKNESS))
    return offset(plate, DISTANCE, GeomAbs_Intersection)
