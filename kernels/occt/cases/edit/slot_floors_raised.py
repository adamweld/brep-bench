"""A comb's two slot floors raised, one until its slot is gone. A comb: the block from the origin to
(40, 15, 20) with two slots cut down into its top, right across it along y: the boxes from (8, 0, 8)
to (14, 15, 20) and from (26, 0, 8) to (32, 15, 20) taken away. Each slot is 6 wide and 12 deep,
its floor at z = 8.

Then edit it directly, moving only the two slots' floors, each along its outward normal, +z, and
leaving every other face where it is: the first slot's floor (8 ≤ x ≤ 14) by 5, up to z = 13, and
the second's (26 ≤ x ≤ 32) by 12, up to z = 20. The slots' walls are trimmed to meet the moved
floors. The second floor arrives in the plane of the top, so its slot's walls shrink to nothing and
the slot is gone: the top runs on across it, one face from x = 14 to x = 40.

The result is the block from the origin to (40, 15, 20) with one slot, the box from (8, 0, 13) to
(14, 15, 20) taken away."""
from bench import face_through
from edit import box, less, moved


def build():
    comb = less(less(box((0, 0, 0), (40, 15, 20)), box((8, -1, 8), (14, 16, 21))),
                box((26, -1, 8), (32, 16, 21)))
    first = face_through(comb, (11, 7.5, 8))
    second = face_through(comb, (29, 7.5, 8))
    return moved(comb, [(first, 5), (second, 12)])
