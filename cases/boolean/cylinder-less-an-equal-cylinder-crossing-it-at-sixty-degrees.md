+++
title = "A cylinder less an equal cylinder crossing it at sixty degrees"
tags = ["boolean", "cut", "analytic", "singular", "cylinder", "pieces"]
origin = "OCCT tests/boolean/boptuc_simple/ZE2"

[volume]
value = 399.20980011955056
exact = "π·3²·20 less the common part of two equal cylinders whose axes meet at angle θ, 16r³/(3 sin θ) = 96√3 for r = 3, θ = 60°"
tex = '\pi r^2 L - \dfrac{16 r^3}{3\sin\theta},\quad r = 3,\ L = 20,\ \theta = 60^\circ'

[area]
value = 433.5397861953915
exact = "the second cylinder's own 2π·3·20 + 2π·3²; the wall it loses inside the first and the wall of the first it gains inside the second are equal, each 8r²/sin θ, and cancel"
tex = '2\pi r L + 2\pi r^2 + \tfrac{8r^2}{\sin\theta} - \tfrac{8r^2}{\sin\theta} = 138\pi'
+++

Two cylinders of radius 3 and length 20, with axes that cross at their midpoints, the origin, at an angle of 60°. The first lies on the x axis, from x = −10 to x = 10. The second lies along the horizontal direction (cos 60°, sin 60°, 0) = (1/2, √3/2, 0), from −10 to 10 along it. Cut the first away from the second.

The cylinders have the same radius, so their walls meet in two ellipses, in the two planes through the z axis that bisect the axes, and the ellipses cross at the two points (0, 0, ±3), where the walls touch. The first cylinder passes right through the second, so what is left is the two ends of the second, each with a saddle-shaped concave face where the first's wall was: two separate pieces. Build it all as one result and measure it whole.

**As a set:**

With u = x/2 + (√3/2)y, the distance squared of a point from the second's axis is x² + y² + z² − u², so

$$\{\,x^2 + y^2 + z^2 - u^2 \le 9,\ |u| \le 10\,\} \setminus \{\,y^2 + z^2 \le 9,\ |x| \le 10\,\}$$
