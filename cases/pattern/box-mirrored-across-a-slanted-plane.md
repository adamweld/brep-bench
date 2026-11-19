+++
title = "A box joined to its mirror image across a slanted plane through it"
tags = ["pattern", "mirror", "union", "planar", "analytic"]
origin = "FreeCAD src/Mod/Part/TestPartApp.py testIssue2671"

[volume]
value = 2958.3333333333335
exact = "10·(2·200 − 625/6) = 8875/3: the section is the two rectangles less their overlap, the hexagon of area 625/6"
tex = '10\left(2\cdot 200 - \tfrac{625}{6}\right) = \tfrac{8875}{3}'

[area]
value = 1375.0
exact = "2(400 − 625/6) + 10(60 + 60 − 125/3) = 1375: the boundary of a union of two convex regions is theirs less their overlap's, of perimeter 4·25/6 + 2·25/2 = 125/3"
tex = '2\left(400 - \tfrac{625}{6}\right) + 10\left(60 + 60 - \tfrac{125}{3}\right) = 1375'
+++

A box from the origin to (20, 10, 10), joined to its mirror image in the upright plane
x + 2y = 25: the plane through (5, 10, 0) with normal (1, 2, 0), which crosses the box near its
corner (20, 10).

The mirror takes p = (x, y, z) to p − (2(x + 2y − 25)/5)·(1, 2, 0). So the image is a box of the
same size, turned in the plane of x and y, from z = 0 to z = 10, its corners at (x, y) = (10, 20),
(22, 4), (14, −2) and (2, 14). It pokes out through the box's bottom side y = 0 and its side x = 20.

The box and its image overlap in a prism about the plane whose section is the hexagon (25/2, 0),
(50/3, 0), (20, 5/2), (20, 20/3), (35/2, 10), (5, 10). The solid is their union, one solid: a
prism 10 tall whose section is the rectangle and the turned rectangle together, with fourteen
sides.

**As a set:**

$$K \cup M(K), \quad K = [0, 20] \times [0, 10] \times [0, 10]$$

M is the mirror in the plane x + 2y = 25.
