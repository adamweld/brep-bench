"""A block's two rounded top edges deleted: the top and both sides meet again. A block, the box from the origin to (20, 10, 10), with the two long edges of its top rounded to
radius 2: the edge where the top, z = 10, meets the side y = 0, and the one where it meets the side
y = 10. Each round is a quarter cylinder: the first about the line y = 2, z = 8, tangent to the top
along y = 2 and to its side along z = 8; the second about the line y = 8, z = 8, tangent to the top
along y = 8 and to its side along z = 8.

Then edit the block directly: delete both rounds together. Deleting a face takes it out of the solid
and closes the gap it leaves with the faces round it, each run on along its own surface until they
meet. The top runs on from y = 2 and from y = 8, and the two sides up from z = 8, until they meet in
the edges y = 0, z = 10 and y = 10, z = 10, and the two ends, x = 0 and x = 20, get their corners
back.

The result is the box from the origin to (20, 10, 10)."""
import math

from bench import face_through
from blend import edges_where, rounded
from edit import box, deleted, middle_at


def build():
    block = box((0, 0, 0), (20, 10, 10))
    edges = edges_where(block, middle_at((10, 0, 10))) + edges_where(block, middle_at((10, 10, 10)))
    block = rounded(block, 2, edges, "the block's two top edges")
    # Each round at its middle, 45° round its axis (y = 2 or y = 8, z = 8).
    s = math.sqrt(2)
    return deleted(block, [face_through(block, (10, 2 - s, 8 + s)),
                           face_through(block, (10, 8 + s, 8 + s))])
