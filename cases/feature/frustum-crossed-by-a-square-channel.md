+++
title = "A cone frustum with a square channel cut across it, off its axis"
tags = ["feature", "channel", "cut", "cone", "planar"]
settled = false
origin = "OCCT feat/featprism/R8"
+++

A cone frustum on the z axis, radius 12 at z = 0 and radius 6 at z = 14, with a channel of square
section cut right across it. The channel's section is the square 1 ≤ y ≤ 7, 3 ≤ z ≤ 9, and it runs
along x from the frustum's near wall to its far wall, off the axis, so it leaves by a mouth in the
conical wall at each end. At z = 9 the frustum's radius is 12 − 6·9/14 ≈ 8.14, more than 7, so
the channel never breaks out through the frustum's side.

**As a set:**

$$\{\,\rho \le 12 - \tfrac{3}{7}z,\ 0 \le z \le 14\,\} \setminus \bigl(\mathbb{R} \times [1,7] \times [3,9]\bigr)$$

Its volume is a double integral of $2\sqrt{r(z)^2 - y^2}$ over the channel's section, and its area has
no short closed form; neither is given here.
