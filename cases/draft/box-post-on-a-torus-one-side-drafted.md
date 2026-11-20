+++
title = "A square post on a torus's tube, one side drafted"
tags = ["draft", "planar", "torus", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/F7"
+++

The torus: the solid swept by the disk of radius 5 centered on (15, 0, 0) in the plane y = 0,
turned a full turn about the z axis. The middle circle of its tube lies in the plane z = 0, and it
reaches up to z = 5.

A post stands on its tube: the box [11, 19] × [−4, 4] × [0, 14], united with the torus. The box's
floor lies inside the tube; its sides show only above the tube's surface, and its top, at z = 14,
is well above the torus.

Draft the post's side face in the plane x = 19 by 4°, with the pull direction +z and the neutral
plane z = 0, the plane of the torus's middle: the face turns about the line x = 19, z = 0, until it
leans 4° from the z direction, toward the post's inside as it rises. It becomes the plane
x = 19 − z·tan 4°. The post's other three sides, its top and the torus are not drafted; the top and
the two sides next to the drafted face are cut back to meet it. It has no closed form that has been checked.

**As a set:**

$$\{\, (\rho - 15)^2 + z^2 \le 25 \,\} \cup \bigl(\,[11,19] \times [-4,4] \times [0,14] \cap \{\, x \le 19 - z\tan 4^\circ \,\}\bigr)$$
