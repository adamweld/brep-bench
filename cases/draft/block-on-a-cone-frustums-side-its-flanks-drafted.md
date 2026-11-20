+++
title = "A block standing out of a cone frustum's side, its flanks drafted"
tags = ["draft", "planar", "cone", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/E4"
+++

A frustum of a cone about the z axis, from z = 0 to z = 20, its section at height z the disk of
radius 10 − z/5 (so 10 at the floor and 6 at the top), and on its side a block from x = 0 to
x = 16, y = −3 to y = 3 and z = 5 to z = 15, united with it. The block stands out of the cone's
wall; its part inside is part of the frustum.

Draft the block's two flanks, its faces y = 3 and y = −3, by 5°, with the pull direction +z and the
neutral plane z = 5, the block's floor: each turns about its line at z = 5 until it leans 5° from the
z direction, inward as it rises, y = ±(3 − (z − 5)·tan 5°). The end face x = 16, the block's floor
and top and the frustum are not drafted; the top is trimmed to the flanks, and each flank ends on
the cone's wall in a curve. It has no closed form that has been checked.

**As a set:**

$$\{\,\rho \le 10 - z/5,\ 0 \le z \le 20\,\} \cup \{\,0 \le x \le 16,\ |y| \le 3 - (z-5)\tan 5^\circ,\ 5 \le z \le 15\,\}$$
