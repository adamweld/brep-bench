"""An open slot in a block's side deleted: the side closes over it. A block, the box from the origin
to (20, 10, 10), with a slot cut into its side y = 0, top to bottom: 4 wide, from x = 8 to x =
12, running in to y = 5 and closed there by a half cylinder of radius 2 about the vertical line
through (10, 5). The slot has three faces: the flats x = 8 and x = 12, and the half cylinder,
tangent to both.

Then edit the block directly: delete all three. Deleting a face takes it out of the solid and closes
the gap it leaves with the faces round it, each run on along its own surface until they meet. The
side y = 0 runs on across the slot's mouth, and the top and the bottom close in over the slot's
outline.

The result is the box from the origin to (20, 10, 10)."""
from bench import face_through
from edit import box, deleted, drilled, less


def build():
    # The slot's straight part, run on past the side, the top and the bottom, and its round end.
    opened = less(box((0, 0, 0), (20, 10, 10)), box((8, -1, -1), (12, 5, 11)))
    slotted = drilled(opened, 10, 5, 2, -1, 11)
    picks = [(8, 2, 5), (12, 2, 5), (10, 7, 5)]
    return deleted(slotted, [face_through(slotted, point) for point in picks])
