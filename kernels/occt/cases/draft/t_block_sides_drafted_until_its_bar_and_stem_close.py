"""A T-block with its sides drafted until they close over its bar and its stem. The outline through
(0, 0), (20, 0), (20, 4), (13, 4), (13, 16), (7, 16), (7, 4), (0, 4) in the plane z = 0, extruded
from z = 0 to z = 8; every side drafted 45 degrees, pull +z, about the neutral plane z = 0, inward
as it rises. The sides close before the top, so the solid is what lies under them above the floor.

BRepOffsetAPI_DraftAngle declines these sides (it cannot recompute the edges once the sides close
over the concave edge), so the solid is made of two boxes, the bar from (0, 0) to (20, 4) and the
stem, run down through the bar, from (7, 0) to (13, 16), each cut by the four planes its sides would
be drafted to, and fused: under the sides' planes the block's section at every height is the union
of those two rectangles' sections."""
from draft import leaning_boxes_fused

RECTANGLES = [((0, 0), (20, 4)), ((7, 0), (13, 16))]
HEIGHT = 8
DRAFT = 45  # degrees, inward as the sides rise


def build():
    return leaning_boxes_fused(RECTANGLES, HEIGHT, DRAFT)
