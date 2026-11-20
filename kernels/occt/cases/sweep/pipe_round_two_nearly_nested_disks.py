"""A thin round pipe along the outline of two nearly nested disks. A pipe of circular section swept along the closed outline of the union of two disks, one almost
inside the other, their circles crossing at a shallow angle, its corners made round.

The two disks lie in the plane z = 0: the first has radius 10 and is centered on the origin, the
second has radius 9.5 and is centered on (0.6, 0, 0). The second would lie inside the first if it
were centered 0.5 or less from the origin; at 0.6 its circle pokes out through the first's by 0.1
at x = 10.1, and the two circles cross at two points, (8.425, ±5.387, 0) to three places. The path
is the outline of the union of the two disks: the arc of the first circle outside the second disk
(the long way round, through (−10, 0, 0)) and the short arc of the second circle outside the first
disk (through (10.1, 0, 0)), joined end to end at the two crossing points, where the outline has
two corners, each a shallow turn of about 2°, 10.8 apart.

The pipe's section is a disk of radius 0.25, centered on the path and square to it. At each corner
the section is turned about the corner (the corner made round), so the pipe is every point within
0.25 of the outline: a ring."""
from pipes import tube_round_two_arcs


def build():
    return tube_round_two_arcs(10, 9.5, 0.6, 0.25)
