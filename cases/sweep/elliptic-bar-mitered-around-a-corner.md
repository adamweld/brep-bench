+++
title = "An elliptic bar mitered around a right-angled corner"
tags = ["sweep", "planar-path", "corner", "miter", "analytic"]
origin = "OCCT pipe/standard/B5"

[volume]
value = 6031.8578948924030
exact = "πab·L = π·6·4·80 = 1920π (each run, cut by the miter's plane through the path's corner, holds its section's area times its length on the path)"
tex = '\pi a b\,L = \pi\cdot 6\cdot 4\cdot 80 = 1920\pi'

[area]
value = 2689.2667816588044
exact = "80P + 2πab, P = 24·E(5/9) the ellipse's perimeter (E the complete elliptic integral of the second kind, parameter 1 − 4²/6²): each run's wall, cut obliquely by the miter plane through the ellipse's center, is the perimeter times the run's length on the path (the perimeter's centroid is the ellipse's center); the miter is inside; the two ends are ellipses of area πab"
tex = '80P + 2\pi a b,\quad P = 24\,E\!\left(\tfrac59\right),\ a = 6,\ b = 4'
+++

A bar of elliptic section swept along a path with one right-angled corner, mitered there.

The path: two straight runs, from (0, 0, 0) along +x to (40, 0, 0), then turning a right angle to
the left (toward +y) and on 40 to (40, 40, 0).

The section: an ellipse square to the path and centered on it, with semi-axis 6 along z and
semi-axis 4 across the run, in the xy plane.

Each run is the ellipse swept straight along it, and at the corner the two are mitered: each is
cut by, and they meet in, the vertical plane through the corner that bisects it, x + y = 40. The
bar's ends are flat and square to the path: in the plane x = 0, and in the plane y = 40.

**As a set:**

$$\Bigl\{\,x \ge 0,\ x + y \le 40,\ \bigl(\tfrac{y}{4}\bigr)^2 + \bigl(\tfrac{z}{6}\bigr)^2 \le 1\Bigr\} \cup \Bigl\{\,y \le 40,\ x + y \ge 40,\ \bigl(\tfrac{x - 40}{4}\bigr)^2 + \bigl(\tfrac{z}{6}\bigr)^2 \le 1\Bigr\}$$
