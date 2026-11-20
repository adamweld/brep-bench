+++
title = "A cube less an equal cube standing apart from it"
tags = ["boolean", "cut", "planar", "disjoint"]
origin = "FreeCAD tests/src/Mod/Part/App/FeaturePartCut.cpp::testNonIntersecting"

[volume]
value = 1000
exact = "10³: the tool touches nothing, so the whole cube is left"
tex = '10^3'

[area]
value = 600
exact = "6·10²"
tex = '6\cdot 10^2'
+++

A cube of side 10 with one corner at the origin, spanning x, y and z from 0 to 10. A second cube of
the same size, shifted along x by 12, so that it spans x from 12 to 22 and y and z from 0 to 10.
The two are 2 apart, a gap of 2 between the face x = 10 of the first and the face x = 12 of the
second. Cut the second from the first. The tool is nowhere near the cube, so the result is the first
cube, whole.

**As a set:**

$$\bigl([0,10]^3\bigr) \setminus \bigl([12,22]\times[0,10]^2\bigr)$$
