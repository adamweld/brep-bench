+++
title = "A circle swept along a parabola arc"
tags = ["sweep", "planar-path", "conic", "analytic"]
origin = "OCCT pipe/standard/A8, OCCT pipe/standard/A6"

[volume]
value = 297.35910504583701
exact = "πr²L, r = 2, L = 8√5 + 4·asinh 2: a tube whose radius is below the path's least radius of curvature (8, at the vertex) and whose sections never meet holds πr² per unit length. L = ∫₀¹⁶ √(1 + (x/8)²) dx = 8·[t√(1+t²)/2 + asinh(t)/2] from t = 0 to 2"
tex = '\pi r^2 L,\quad r = 2,\ L = \int_0^{16}\sqrt{1 + (x/8)^2}\,dx = 8\sqrt5 + 4\,\operatorname{asinh} 2'

[area]
value = 322.49184627455329
exact = "2πrL + 2πr²: the wall holds 2πr per unit length, and the two ends are disks"
tex = '2\pi r L + 2\pi r^2'
+++

A circle of radius 2 swept along an arc of a parabola. The arc lies in the xy plane: the curve
y = x²/16, from the origin to (16, 16, 0), whose radius of curvature is least at the origin, 8.

At every point of the arc the section is the disk of radius 2 centered on the arc, in the plane
square to the arc there. The pipe's two ends are flat disks, square to the arc at its ends: in the
plane x = 0, and in the plane through (16, 16, 0) square to the arc there.

**As a set:**

$$\Bigl\{\,\gamma(s) + v \;:\; 0 \le s \le 16,\ v \perp \gamma'(s),\ |v| \le 2\,\Bigr\}, \quad \gamma(s) = (s,\ s^2/16,\ 0)$$
