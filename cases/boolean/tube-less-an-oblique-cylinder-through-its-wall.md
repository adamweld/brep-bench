+++
title = "A thick tube pierced through both walls by an oblique cylinder"
tags = ["boolean", "cut", "cylinder", "oblique"]
origin = "OCCT tests/bugs/modalg_7/bug24905"
settled = false
+++

A tube: the cylinder of radius 5 about the z axis from z = 0 to z = 20, less the coaxial cylinder
of radius 3 bored right through it, so a wall 2 thick.

A cylinder of radius 2 whose axis passes through the point (1, 0, 10) with direction
$(3, 1, 2)/\sqrt{14}$, so that it leans on all three axes and does not cross the z axis. It runs
40 long, 20 either way from that point, far past the tube on both sides, and it has flat ends.

Cut the tube by it: the cylinder passes through the near wall, across the bore, and through the
far wall, leaving an oblique hole in each; the tube's flat ends are not reached. The result is the
tube, one solid, less the cylinder.

The two holes' edges are space curves with no closed form for the volume, so the case is unsettled
until two kernels agree.

**As a set:**

$$\{\,3 \le \rho \le 5,\ 0 \le z \le 20\,\} \setminus \{\,d\bigl(p, \text{the axis}\bigr) \le 2\,\}$$

where the axis is the line through (1, 0, 10) with direction (3, 1, 2).
