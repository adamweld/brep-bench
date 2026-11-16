+++
title = "A cone frustum's flat deleted: the frustum is whole again"
tags = ["edit", "delete-face", "cone", "planar", "analytic"]

[volume]
value = 1319.4689145077132
exact = "(π·20/3)(6² + 6·3 + 3²) = 420π"
tex = '\tfrac{\pi\cdot 20}{3}\left(6^2 + 6\cdot 3 + 3^2\right) = 420\pi'

[area]
value = 713.1846844817552
exact = "π(6² + 3²) + π(6 + 3)√(20² + 3²) = 45π + 9π√409"
tex = '\pi(6^2 + 3^2) + \pi(6 + 3)\sqrt{20^2 + 3^2} = 45\pi + 9\pi\sqrt{409}'
+++

A cone frustum about the z axis, of radius 6 at z = 0 narrowing to radius 3 at z = 20 (radius
6 − 3z/20 at height z), with a flat milled along its side: everything with y < −4 cut away. The cone
is wider than 4 only below z = 40/3, so the flat runs up from the base and ends where the cone
narrows past it, its edge on the cone a hyperbola.

Then edit the frustum directly: delete the flat. Deleting faces takes them out of the solid and closes the gap they leave with the faces round them,
each run on along its own surface until they meet. The flat borders the cone and the base, so
the cone closes over it and the base grows back to a whole disc.

The result is the plain frustum, of radius 6 at z = 0 and 3 at z = 20.

**As a set:**

$$\{\,\sqrt{x^2 + y^2} \le 6 - \tfrac{3z}{20},\ 0 \le z \le 20\,\}$$
