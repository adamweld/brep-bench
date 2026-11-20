+++
title = "A ball less a drum tangent along its equator"
tags = ["tangent", "boolean", "cut", "analytic", "sphere", "cylinder", "tangent-circle"]
origin = "OCCT tests/boolean/boptuc_simple/ZH5"

[volume]
value = 134.04128655316451
exact = "half a ball, 2πr³/3, r = 4"
tex = '\tfrac{2}{3}\pi r^3,\quad r = 4'

[area]
value = 150.79644737231007
exact = "the half sphere 2πr² and the flat disc πr²"
tex = '2\pi r^2 + \pi r^2 = 3\pi r^2,\quad r = 4'
+++

A ball of radius 4 centred on the origin, and a drum: a cylinder of radius 4 with the z axis as its axis, from z = −6 to z = 0. The drum's top face is the plane z = 0 through the ball's centre, and its wall has the ball's radius, so it is tangent to the ball all round the equator, where the ball's wall meets the drum's top rim and the two walls continue smoothly into each other. The ball's lower half lies inside the drum, which is deeper than the ball's radius. Cut the drum away from the ball.

What is left is the upper half of the ball: a hemisphere of radius 4 with a flat disc where the drum's top face was.

**As a set:**

$$\{\,x^2 + y^2 + z^2 \le 4^2\,\} \setminus \{\,\rho \le 4,\ -6 \le z \le 0\,\}$$
