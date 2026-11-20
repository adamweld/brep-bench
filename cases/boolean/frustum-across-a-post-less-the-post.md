+++
title = "A frustum lying across a post, less the post: its two ends"
tags = ["boolean", "cut", "cone", "cylinder", "pieces", "cone-cylinder"]
origin = "OCCT tests/boolean/boptuc_simple/ZK5"

[volume]
value = 46.612066414343829
exact = "∫ over the frustum's length of the part of its section outside the post: the whole disc πr(x)² where |x| ≥ 4, and elsewhere two circular segments 2(r² arccos(s/r) − s√(r² − s²)) with s = √(16 − x²), when s < r; r(x) = 3/2 − x/14 (the post's wall is cut by the frustum only for x from about −3.594 to 3.807)"
tex = '\int_{-7}^{7} A_{\rm out}(x)\,dx'

[area]
value = 90.24956377734332
exact = "numerically, three parts: the frustum's wall outside the post, ∫ r√(1 + r'²)·θ(x) dx with θ = 4 arccos(s/r) or 2π; its two ends π·2² + π·1²; and the post's wall inside the frustum, ∫ 8√(r(4cos φ)² − 16 sin² φ) dφ over the angles where that is real"
tex = '\int r\sqrt{1 + r^{\prime 2}}\,\theta\,dx + 5\pi + \int 8\sqrt{r^2 - 16\sin^2\varphi}\,d\varphi'
+++

A post of radius 4 on the z axis, from z = −6 to z = 6.

A frustum of a cone on the x axis, lying across the post at right angles through its mid-height, so the axes cross at the origin: from x = −7 to x = 7, radius 2 at x = −7 narrowing in a straight line to radius 1 at x = 7 (radius 3/2 − x/14 at x). It passes right through the post, out of both sides, and is clear of the post's top and bottom. Cut the post away from the frustum.

What is left is two separate pieces, the frustum's two ends outside the post's wall, each with a hollow face where the post's wall was. The frustum's wall crosses the post's wall along a closed curve on each side.

**As a set:**

$$\{\,|x| \le 7,\ y^2 + z^2 \le (\tfrac32 - \tfrac x{14})^2\,\} \setminus \{\,x^2 + y^2 \le 16,\ |z| \le 6\,\}$$
