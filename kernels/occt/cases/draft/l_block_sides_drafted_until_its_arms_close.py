"""An L-block with its sides drafted until they close over its arms. The outline through (0, 0),
(20, 0), (20, 5), (5, 5), (5, 15), (0, 15) in the plane z = 0, extruded from z = 0 to z = 8; every
side drafted 30 degrees, pull +z, about the neutral plane z = 0, inward as it rises. The sides close
before the top, so the solid is what lies under them above the floor.

BRepOffsetAPI_DraftAngle declines these sides (it cannot recompute the edges once the sides close
over the concave edge), so the solid is made of two boxes, the bar from (0, 0) to (20, 5) and the
column from (0, 0) to (5, 15), each cut by the four planes its sides would be drafted to, and fused:
under the sides' planes the block's section at every height is the union of those two rectangles'
sections."""
from draft import leaning_boxes_fused

RECTANGLES = [((0, 0), (20, 5)), ((0, 0), (5, 15))]
HEIGHT = 8
DRAFT = 30  # degrees, inward as the sides rise


def build():
    return leaning_boxes_fused(RECTANGLES, HEIGHT, DRAFT)
