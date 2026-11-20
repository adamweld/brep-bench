+++
title = "A circle swept along a quarter of an ellipse"
tags = ["sweep", "planar-path", "conic", "analytic"]
origin = "OCCT pipe/standard/A5"

[volume]
value = 304.37157754357684
exact = "πr²L, r = 2, L = 20·E(3/4) (a quarter of the ellipse's perimeter, E the complete elliptic integral of the second kind with parameter m = 1 − (10/20)²): a tube whose radius is below the path's least radius of curvature (5, at (20, 0, 0)) and whose sections never meet holds πr² per unit length"
tex = '\pi r^2 L,\quad r = 2,\ L = 20\,E\!\left(\tfrac34\right)'

[area]
value = 329.50431877295190
exact = "2πrL + 2πr²: the wall holds 2πr per unit length, and the two ends are disks"
tex = '2\pi r L + 2\pi r^2'
+++

A circle of radius 2 swept along a quarter of an ellipse. The path lies in the xy plane: the ellipse
with semi-axes 20 along x and 10 along y, centered on the origin, from (20, 0, 0) to (0, 10, 0)
(the points (20 cos t, 10 sin t, 0), t from 0 to a quarter turn). Its radius of curvature is least
at (20, 0, 0), 5, and greatest at (0, 10, 0), 40.

At every point of the path the section is the disk of radius 2 centered on it, in the plane square
to the path there. The pipe's two ends are flat disks, square to the path at its ends: in the plane
y = 0, and in the plane x = 0.

**As a set:**

$$\Bigl\{\,\gamma(t) + v \;:\; 0 \le t \le \tfrac{\pi}{2},\ v \perp \gamma'(t),\ |v| \le 2\,\Bigr\}, \quad \gamma(t) = (20\cos t,\ 10\sin t,\ 0)$$
