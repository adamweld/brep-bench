+++
title = "A ball united with a cylinder across it whose top line just touches the ball at its pole"
tags = ["boolean", "union", "sphere", "cylinder", "tangent", "point-contact"]
origin = "OCCT bugs/modalg_7/bug32502"
settled = false
+++

A ball of radius 5 about the origin, and a cylinder of radius 2 lying along the x axis direction, its axis the line through (0, 0, 3) parallel to the x axis, from x = −8 to x = 8. Make their union.

The cylinder's axis passes 3 above the ball's centre, which is the ball's radius less the cylinder's, so the cylinder's topmost line, z = 5, just touches the ball's surface at its pole (0, 0, 5), and only there. The cylinder's wall otherwise crosses the ball's surface in a curve of two loops, one on each side of the plane x = 0, that meet at the pole.

**As a set:**

$$B_5(0, 0, 0) \cup \{\,y^2 + (z - 3)^2 \le 2^2,\ |x| \le 8\,\}$$
