+++
title = "Two holes widened until they merge"
tags = ["edit", "offset-face", "hole", "cylinder", "analytic", "self-intersection"]

[volume]
value = 7862.467521335166
exact = "8(1200 − U), U = 72(π − α) + 10√11, α = acos(5/6); the union of two radius-6 disks whose centers are 10 apart"
tex = '8\left(1200 - 72(\pi-\alpha) - 10\sqrt{11}\right),\quad \alpha=\arccos\tfrac56'

[area]
value = 3576.3510454792586
exact = "2(1200 − U) + 140·8 + 24(π − α)·8 = 3520 + 48(π − α) − 20√11, α = acos(5/6)"
tex = '3520 + 48(\pi-\alpha) - 20\sqrt{11},\quad \alpha=\arccos\tfrac56'
+++

A plate, the box from the origin to (40, 30, 8), drilled right through along z by two holes of
radius 3, with axes through (15, 15) and (25, 15).

Then edit it directly: offset both cylindrical hole walls by 3 into the plate, away from their
axes, so that each becomes a cylinder of radius 6. The two holes now overlap. Trim the top and
bottom to their combined opening, and discard the parts of either cylindrical wall inside the
other hole. There is no wall or sheet between the holes.

The result is the original box less the union of two through-cylinders of radius 6 about those
same axes. It is one body with one through-opening. Its two surviving cylindrical walls meet
along the vertical lines through (20, 15 − √11) and (20, 15 + √11).

Put α = acos(5/6), in radians. The overlapping disks have intersection area 72α − 10√11,
so their union has area U = 72(π − α) + 10√11 and perimeter 24(π − α). The volume is
the remaining planar area times 8. The surface area is the two planar ends, the box's outside
walls of area 140·8, and the opening's perimeter times 8.
