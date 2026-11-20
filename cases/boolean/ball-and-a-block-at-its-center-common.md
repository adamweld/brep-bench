+++
title = "A ball and a block with a corner at its center, their common part"
tags = ["boolean", "common", "analytic", "sphere", "seam"]
origin = "OCCT boolean/bcommon_simple/A1, OCCT boolean/bcommon_simple/A2, OCCT boolean/bcommon_simple/A4"

[volume]
value = 0.79796453401180748
exact = "πR³/6 − 3·(π/4)(R²(R − 1) − (R³ − 1)/3), R = 1.2"
tex = '\tfrac{\pi R^3}{6} - 3\cdot\tfrac{\pi}{4}\Bigl(R^2(R-1) - \tfrac{R^3-1}{3}\Bigr),\quad R = 1.2'

[area]
value = 5.0204323233322818
exact = "πR²/2 − 3πR(R − 1)/2 + 3(πR²/4 − 2S) + 3π(R² − 1)/4, S = πR²/4 − ½√(R² − 1) − (R²/2)·asin(1/R)"
tex = '\tfrac{\pi R^2}{2} - \tfrac{3\pi R(R-1)}{2} + 3\Bigl(\tfrac{\pi R^2}{4} - 2S\Bigr) + \tfrac{3\pi(R^2-1)}{4},\quad S = \tfrac{\pi R^2}{4} - \tfrac12\sqrt{R^2-1} - \tfrac{R^2}{2}\arcsin\tfrac1R'
+++

A ball of radius 1.2 centered on the origin and a cube of side 1 from (0, 0, 0) to (1, 1, 1), both with their natural faces. Make the solid common to both.

The cube's corner at the origin is the ball's center, and its three edges from there run along the coordinate axes. The cube's corners (1, 0, 0), (0, 1, 0) and (0, 0, 1) lie inside the ball, at distance 1, and its other four corners lie outside it, at distances √2 and √3, both more than 1.2. The common part is the octant of the ball with the cube's three far faces, the planes x = 1, y = 1 and z = 1, cutting a small cap off it each: it is bounded by three flat faces in the planes x = 0, y = 0 and z = 0, three small flat faces in x = 1, y = 1 and z = 1, and one piece of the sphere. The solid is the same wherever the ball's poles and seam are put, so a kernel's sphere may be turned any way.

**As a set:**

$$B_{1.2} \cap [0,1]^3$$
