+++
title = "A square post on a dome, one side drafted"
tags = ["draft", "planar", "sphere", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/K1"
+++

The dome: everything within 20 of the origin with z ≥ 0, a hemisphere whose flat face is the disk
z = 0 and whose pole is (0, 0, 20).

A post stands on it: the box [−5, 5] × [−5, 5] × [0, 30], united with the dome. Its sides show only
above the sphere, where the sphere is under it; at its edges that is from z ≈ 18.7 up.

Draft the post's side face in the plane x = 5 by 4°, with the pull direction +z and the neutral
plane z = 0, the dome's flat base: the face turns about the line x = 5, z = 0, until it leans 4°
from the z direction, toward the post's inside as it rises. It becomes the plane
x = 5 − z·tan 4°. The post's other three sides, its top and the dome are not drafted; the top and
the two sides next to the drafted face are cut back to meet it.

So the result is the dome united with the box cut to x ≤ 5 − z·tan 4°. It has no closed form that has been checked.

**As a set:**

$$\bigl(B_{20} \cap \{\, z \ge 0 \,\}\bigr) \cup \bigl(\,[-5,5]^2 \times [0,30] \cap \{\, x \le 5 - z\tan 4^\circ \,\}\bigr)$$
