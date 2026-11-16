"""A blind bore's beveled floor deleted: the floor and the wall meet again. A block, the box from (−10, −10, 0) to (10, 10, 10), with a blind bore of radius 4 about the z axis sunk from its
top down to a floor at z = 4; but the concave edge where the floor meets the bore's wall is beveled: the wall
stops at z = 5 and a cone runs from there down and in to the floor, now the disk of radius 3 at z = 4. The
cutter is the cylinder of radius 4 from z = 5 up through the top joined to the frustum from radius 3 at z = 4
to radius 4 at z = 5.

Then edit it directly: delete the bevel. Deleting a face takes it out of the solid and closes the gap it
leaves with the faces round it, each run on along its own surface until they meet. The floor runs on outward
from radius 3 and the wall down from z = 5 until they meet in the circle of radius 4 at z = 4.

The result is the block with the sharp-floored bore."""
from edit import box, cylinder, deleted, frustum, less, united
from bench import face_through


def build():
    cutter = united(cylinder((0, 0, 5), 4, 6), frustum((0, 0, 4), 3, 4, 1))
    beveled = less(box((-10, -10, 0), (10, 10, 10)), cutter)
    # The bevel, halfway along its slant.
    return deleted(beveled, [face_through(beveled, (0.0, -3.5, 4.5))])
