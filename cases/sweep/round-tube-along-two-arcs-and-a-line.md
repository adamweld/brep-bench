+++
title = "A round tube swept along a tight arc, a flatter reverse arc and a straight run"
tags = ["sweep", "planar-path", "analytic", "tangent", "torus"]
origin = "OCCT bugs/modalg_7/bug32447_3, OCCT bugs/modalg_7/bug32447_4"

[volume]
value = 9.758397791009363
exact = "πr²L, r = 1/2, L = 2·π/2 + 8·π/4 + 3 = 3π + 3, the path's length (the section is a disc square to the path, centred on it, and r is less than the tightest bend's radius 2)"
tex = '\pi r^2 L = \tfrac{\pi}{4}(3\pi + 3)'

[area]
value = 40.60438749083235
exact = "2πrL + 2πr², the side 2πr·L (the section's circumference carried the path's length) and the two flat ends"
tex = '2\pi r L + 2\pi r^2 = \pi(3\pi + 3) + \tfrac{\pi}{2}'
+++

A tube of circular section swept along a path in the xy plane, made of three pieces each running tangent into the next. The path starts at the origin heading along +x:

1. an arc of radius 2 turning left (anticlockwise) a quarter turn, about the centre (0, 2), to (2, 2), where it is heading along +y;
2. an arc of radius 8 turning right (clockwise) an eighth of a turn, about the centre (10, 2), to the point $(10 - 4\sqrt{2},\ 2 + 4\sqrt{2})$, where it is heading along $(1, 1)/\sqrt{2}$;
3. a straight run of length 3 along that heading.

The section is a disc of radius 1/2, centred on the path and square to it, and the ends are flat. The path is 3π + 3 long.

**As a set:** the points within 1/2 of the path, cut off flat at its ends:

$$\{\,p : \operatorname{dist}(p, \gamma) \le \tfrac12\,\}\ \text{between the planes square to}\ \gamma\ \text{at its ends}$$
