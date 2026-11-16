"""A cone frustum's beveled top rim deleted: the top and the cone meet again. A cone frustum on the z axis, its base the disk of radius 6 at z = 0, its side the cone 2r + z = 12, cut off at
z = 6 where its top would be the disk of radius 3; but its top rim is beveled: the side stops at z = 5 (radius
3.5), and a steeper cone runs from there to the top, the disk of radius 2 at z = 6. That is the frustum from
radius 6 at z = 0 to 3.5 at z = 5 joined to the frustum from radius 3.5 at z = 5 to 2 at z = 6.

Then edit it directly: delete the bevel. Deleting a face takes it out of the solid and closes the gap it
leaves with the faces round it, each run on along its own surface until they meet. The top runs on outward
from radius 2 and the side up from z = 5 until they meet in the circle of radius 3 at z = 6.

The result is the frustum from radius 6 at z = 0 to radius 3 at z = 6."""
from edit import deleted, frustum, united
from bench import face_through


def build():
    beveled = united(frustum((0, 0, 0), 6, 3.5, 5), frustum((0, 0, 5), 3.5, 2, 1))
    # The bevel, halfway along its slant.
    return deleted(beveled, [face_through(beveled, (0.0, -2.75, 5.5))])
