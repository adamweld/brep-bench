+++
title = "A square post on a block under a parabolic dome, one side drafted"
tags = ["draft", "planar", "bezier", "revolve", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/G3"
+++

The block: the solid turned about the z axis from the region under the quadratic Bézier arc from
(ρ, z) = (0, 10) through (10, 10) to (20, 4), above z = 0 and out to ρ = 20. The arc is the
parabola z = 10 − 0.015ρ², so the block is a flat-floored disk of radius 20 under a dome, 10 high at
the axis and 4 high at its rim.

A post stands off its axis: the box [2, 10] × [−4, 4] × [0, 30], united with the block. Its sides
show only above the dome, which is 8 to 10 high under the box.

Draft the post's side face in the plane x = 10 by 4°, with the pull direction +z and the neutral
plane z = 0, the block's floor: the face turns about the line x = 10, z = 0, until it leans 4° from
the z direction, toward the post's inside as it rises. It becomes the plane x = 10 − z·tan 4°. The
post's other three sides, its top and the block are not drafted; the top and the two sides next to
the drafted face are cut back to meet it. It has no closed form that has been checked.

**As a set:**

$$\{\, 0 \le z \le 10 - 0.015\rho^2,\ \rho \le 20 \,\} \cup \bigl(\,[2,10] \times [-4,4] \times [0,30] \cap \{\, x \le 10 - z\tan 4^\circ \,\}\bigr)$$
