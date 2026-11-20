+++
title = "Bars patterned along an L-shaped path, each turned to follow it"
tags = ["pattern", "path", "union", "planar", "coincident", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestPathPattern.py::testReverseAndAlignUseRelativePathFrames"

[volume]
value = 13
exact = "plan area 7 + 7 − 1 (the two strips less their shared corner square) times height 1"
tex = '(7 + 7 - 1)\cdot 1'

[area]
value = 54
exact = "2·13 (top and bottom) + the L's outline, 7 + 7 + 1 + 6 + 6 + 1 = 28, times height 1"
tex = '2\cdot 13 + 28\cdot 1'
+++

A bar 3 long, 1 wide and 1 high: the box from (−0.5, −0.5, 0) to (2.5, 0.5, 1), its length along x.
It is copied six times along an L-shaped path in the plane z = 0, the path running from (0, 0) to
(6, 0) and on to (6, 6): one copy at each distance 1, 3, 5, 7, 9 and 11 along the path from its
start, at the points (1, 0), (3, 0), (5, 0), (6, 1), (6, 3) and (6, 5). Each copy is the bar moved so
that its centre (its footprint's centre, at height 0 to 1) sits on its point, and turned about the
vertical through that centre so that its length lies along the path there: the first three keep the
bar's direction, along x, and the last three, on the second leg, are turned 90° to lie along y. No
copy sits at the corner (6, 0). The six bars are united into one solid.

The first three bars cover the strip −0.5 ≤ x ≤ 6.5, |y| ≤ 0.5, each overlapping the next by 1; the
last three cover the strip |x − 6| ≤ 0.5, −0.5 ≤ y ≤ 6.5, likewise. The two strips share the corner
square 5.5 ≤ x ≤ 6.5, |y| ≤ 0.5, and their outer faces x = 6.5 and y = −0.5 are flush: the
solid is an L-shaped bar 1 high.

**As a set:**

$$\bigl([-0.5,\,6.5]\times[-0.5,\,0.5] \,\cup\, [5.5,\,6.5]\times[-0.5,\,6.5]\bigr)\times[0,1]$$
