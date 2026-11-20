+++
title = "A cube less an equal cube standing against one of its faces"
tags = ["boolean", "cut", "analytic", "planar", "coincident", "touching"]
origin = "FreeCAD tests/src/Mod/Part/App/FeaturePartCut.cpp::testTouching"

[volume]
value = 1000
exact = "the cube itself: 10³"
tex = '10^3'

[area]
value = 600
exact = "the cube's six faces: 6·100"
tex = '6\cdot 10^2'
+++

A cube of side 10 from the origin to (10, 10, 10), and a second cube from (10, 0, 0) to (20, 10, 10), standing against the first's face x = 10: the two faces coincide exactly and the cubes share no volume. Cut the second away from the first.

Nothing is removed; the result is the first cube.

**As a set:**

$$[0,10]^3 \setminus \bigl([10,20]\times[0,10]^2\bigr)$$
