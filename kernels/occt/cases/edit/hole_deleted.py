"""A drilled block's hole deleted: the block is whole again. A block, the box from the origin to
(20, 10, 10), drilled right through along z by a hole of radius 3 whose axis is the vertical line
through (10, 5).

Then edit the block directly: delete the hole's wall. Deleting a face takes it out of the solid and
closes the gap it leaves with the faces round it, each run on along its own surface until they meet.
The wall's two ends are the whole of a circle in the top and a circle in the bottom, so those two
faces close over the openings and nothing else changes.

The result is the box from the origin to (20, 10, 10)."""
from bench import face_through
from edit import box, deleted, drilled


def build():
    block = drilled(box((0, 0, 0), (20, 10, 10)), 10, 5, 3, -1, 11)
    return deleted(block, [face_through(block, (13, 5, 5))])
