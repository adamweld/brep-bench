+++
title = "A post capped by a ball, the cap made turned 1.5 radians about the axis, its rounded rim deleted: the cylinder and the ball meet again"
tags = ["edit", "delete-face", "fillet", "torus", "cylinder", "sphere", "analytic"]

[volume]
value = 356.04716740684324
exact = "πr²h + πc²(3R − c)/3 = 96π + 52π/3 = 340π/3, r = 4, h = 6, R = 5, c = 2"
tex = '\pi r^2 h + \tfrac{1}{3}\pi c^2(3R - c) = \tfrac{340}{3}\pi'

[area]
value = 263.89378290154264
exact = "πr² + 2πrh + 2πRc = 16π + 48π + 20π = 84π"
tex = '\pi r^2 + 2\pi r h + 2\pi R c = 84\pi'
+++

A post on the z axis: the cylinder of radius 4 from z = 0 to z = 6, capped by the part above z = 6 of
the ball of radius 5 about (0, 0, 3), which meets the cylinder's top circle; made as one solid. The
rim where the cylinder and the dome meet, the circle of radius 4 at z = 6, is rounded to radius 1:
the round is a piece of a torus tangent to the cylinder and to the ball.

Then edit it directly: delete the round. Deleting a face takes it out of the solid and closes the
gap it leaves with the faces round it, each run on along its own surface until they meet. The
cylinder runs on up and the dome down until they meet again in the circle of radius 4 at z = 6.

The result is the domed post as it was before its rim was rounded: the cylinder and a cap of height 2
of the ball.

**As a set:**

$$\{\,\sqrt{x^2 + y^2} \le 4,\ 0 \le z \le 6\,\} \cup \{\,x^2 + y^2 + (z - 3)^2 \le 25,\ z \ge 6\,\}$$

Here the dome is made turned by 1.5 radians (85.9437°) about the z axis before the two are joined. That changes no point of the solid, but where each surface's own parameter lines start differs between the two surfaces, so the rim where they meet is two edges and its round two faces; both faces are deleted.
