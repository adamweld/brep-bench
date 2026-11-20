+++
title = "A cylinder less an equal cylinder stacked on it"
tags = ["boolean", "cut", "analytic", "coincident", "face-contact"]
origin = "OCCT tests/boolean/bopcut_simple/ZD4"

[volume]
value = 402.1238596594935
exact = "the cylinder is unchanged: π·4²·8"
tex = '\pi r^2 h = 128\pi,\quad r = 4,\ h = 8'

[area]
value = 301.59289474462014
exact = "2π·4·8 + 2π·4²"
tex = '2\pi r h + 2\pi r^2 = 96\pi'
+++

A cylinder of radius 4 and height 8 standing on the xy plane with its axis the z axis, and a second cylinder of the same radius and height 6 on the same axis from z = 8 to z = 14, stacked on the first so that they share a disc face, their walls running on into one another. Cut the second away from the first. If the cylinders you make have seams, turn the second a quarter turn about the z axis so that the seams do not meet. What is left is the first cylinder, whole.

**As a set:**

$$\{\rho \le 4,\ 0 \le z \le 8\} \setminus \{\rho \le 4,\ 8 \le z \le 14\}$$
