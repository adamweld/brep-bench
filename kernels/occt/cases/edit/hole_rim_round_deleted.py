"""A drilled block's rounded hole rim deleted: the top and the bore meet again. A block, the box from the origin to (20, 10, 10), drilled through from top to bottom by a hole of
radius 2 about the vertical line x = 10, y = 5. The edge where the hole's wall meets the top, z =
10, is rounded to radius 1: the round is a quarter of a torus whose tube has radius 1 about the
circle of radius 3 at z = 9, centered on the hole's axis, tangent to the top along the circle of
radius 3 and to the hole's wall along the circle at z = 9.

Then edit it directly: delete the round. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The top
runs on inward from radius 3 and the hole's wall up from z = 9 until they meet in the circle of
radius 2 at z = 10.

The result is the block drilled through by the hole of radius 2, its rim sharp."""
import math

from bench import face_through
from blend import edges_where, rounded
from edit import box, deleted, drilled, on_circle


def build():
    solid = drilled(box((0, 0, 0), (20, 10, 10)), 10, 5, 2, -1, 11)
    solid = rounded(solid, 1, edges_where(solid, on_circle((10, 5, 10), 2)), "the hole's rim")
    # The round at 45° round its tube, whose center line is the circle of radius 3 at z = 9.
    s = math.sqrt(2) / 2
    return deleted(solid, [face_through(solid, (10, 5 - (3 - s), 9 + s))])
