"""A thin round pipe along the outline of two disks that overlap slightly. A pipe of circular section swept along the closed outline of the union of two disks that cross
slightly, its corners made round.

The two disks lie in the plane z = 0: the first has radius 10 and is centered on the origin, the
second has radius 8 and is centered on (17.9, 0, 0). They overlap by a sliver 0.1 deep along the
x axis, and their circles cross at two points, (9.956, ±0.941, 0) to three places. The path is the
outline of the union of the two disks: the arc of the first circle outside the second disk (all
but a small part round x = 10) and the arc of the second circle outside the first disk (all but
a small part round x = 9.9), joined end to end at the two crossing points, where the outline has
two sharp inward corners, each turning the path through about 168° (the two circles cross at
about 12°), 1.9 apart.

The pipe's section is a disk of radius 0.25, centered on the path and square to it. At each corner
the section is turned about the corner (the corner made round), so the pipe is every point within
0.25 of the outline: a ring. Near each corner the pipes of the two arcs overlap."""
from pipes import tube_round_two_arcs


def build():
    return tube_round_two_arcs(10, 8, 17.9, 0.25)
