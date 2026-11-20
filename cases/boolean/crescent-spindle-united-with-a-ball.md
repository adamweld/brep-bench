+++
title = "A crescent turned about its tips, united with a ball across it"
tags = ["boolean", "union", "revolve", "sphere", "degenerate-edge"]
origin = "OCCT tests/bugs/modalg_7/bug29580_2"
settled = false
+++

In the xz plane, take the disc of radius 5 centered at (0, 0) less the disc of radius 5 centered at
(6, 0): a crescent, whose tips are (3, ±4) and which lies wholly in x ≤ 3. Turn it about the line
x = 3, z = 0, parallel to the y axis (the line through the tips), by 270°, right-handed about +y:
its far side starts on the −x side of the line, goes first toward +z, then +x, and stops at −z.

Unite that solid with the ball of radius 4 centered at (3, 0, 5), above the axis, which crosses the
solid's wall where the turn goes over the top.

**As a set:**

$$\{\,(3 - \rho,\ y) \in C,\ 0 \le \psi \le 270^\circ\,\} \cup \{\,(x-3)^2 + y^2 + (z-5)^2 \le 16\,\}$$

where ρ and ψ are the distance from the axis and the angle turned from its −x side (toward +z), and
C = {(u, y): u² + y² ≤ 25 and (u − 6)² + y² > 25}.
