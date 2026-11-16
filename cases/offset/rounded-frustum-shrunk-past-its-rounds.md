+++
title = "A rounded cone frustum shrunk past its rounds' radius"
tags = ["offset", "shrink", "analytic", "torus", "cone", "round", "vanishing-face", "self-intersection"]

[volume]
value = 32.72736262314977
exact = "πh(a² + ab + b²)/3, h = 6 − 2d, a = (12 − d√5 − d)/2, b = (12 − d√5 − (6 − d))/2, d = 2"
tex = '\tfrac{1}{3}\pi h(a^2 + ab + b^2),\quad h = 6 - 2d,\ a = \tfrac{12 - d\sqrt{5} - d}{2},\ b = \tfrac{12 - d\sqrt{5} - (6 - d)}{2},\ d = 2'

[area]
value = 65.58196621781498
exact = "πa² + πb² + π(a + b)√((a − b)² + h²), with a, b and h as for the volume"
tex = '\pi a^2 + \pi b^2 + \pi(a + b)\sqrt{(a - b)^2 + h^2}'
+++

A cone frustum on the z axis, from the disk of radius 6 at z = 0 to the disk of radius 3 at z = 6 (its side
the cone 2r + z = 12), with both its circular edges rounded to radius 1.5: the top rim by a piece of a torus tangent to the top and to the cone, the bottom rim by one tangent to the bottom and to the cone, each of tube radius 1.5. Shrink it by 2: the solid of every point at least 2 from the
frustum's outside.

That is more than the rounds' radius. A round moved 2 inward would be a torus of tube radius
1.5 − 2 < 0, turned inside out: it is no part of the result, and the moved cone and the moved top or
bottom run on past it until they meet in a sharp edge. The result is the frustum shrunk with sharp
edges: from z = 2 to z = 4, its side the cone 2r + z = 12 − 2√5 (the side moved 2 along its
normal), so its radius is (12 − 2√5 − 2)/2 at the bottom and (12 − 2√5 − 4)/2 at the top.

**As a set:** the rounded frustum lies between the sharp frustum shrunk by 1.5 and grown back by 1.5
and the sharp frustum itself, so for any distance of at least 1.5 its points at least that far inside are
the sharp frustum's:

$$\{\, d \le z \le 6 - d,\ 2\sqrt{x^2 + y^2} + z \le 12 - d\sqrt{5} \,\},\quad d = 2$$
