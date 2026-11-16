"""A cylinder's rounded rims deleted, top and bottom: the ends and the side meet again. A cylinder of radius 6 on the z axis from z = 0 to z = 10, with both its rims rounded to radius 2.
The top round is a quarter of a torus whose tube has radius 2 about the circle of radius 4 at z = 8,
tangent to the top along the circle of radius 4 and to the side along the circle at z = 8; the
bottom round is its mirror image, about the circle of radius 4 at z = 2, tangent to the bottom along
the circle of radius 4 and to the side along the circle at z = 2.

Then edit it directly: delete both rounds together. Deleting a face takes it out of the solid and
closes the gap it leaves with the faces round it, each run on along its own surface until they
meet. The top and the bottom run on outward from radius 4, and the side up from z = 8 and down from
z = 2, until they meet in the circles of radius 6 at z = 10 and at z = 0.

The result is the cylinder of radius 6 from z = 0 to z = 10."""
import math

from bench import face_through
from blend import edges_where, rounded
from edit import cylinder, deleted, on_circle


def build():
    solid = cylinder((0, 0, 0), 6, 10)
    rims = edges_where(solid, on_circle((0, 0, 10), 6)) + edges_where(solid, on_circle((0, 0, 0), 6))
    solid = rounded(solid, 2, rims, "the cylinder's rims")
    # Each round at 45° round its tube, whose center line is the circle of radius 4 at z = 8 or 2.
    s = math.sqrt(2)
    return deleted(solid, [face_through(solid, (0, -(4 + s), 8 + s)),
                           face_through(solid, (0, -(4 + s), 2 - s))])
