+++
title = "A cone frustum's ring groove deleted: the frustum is whole again"
tags = ["edit", "delete-face", "cone", "cylinder", "planar", "groove", "analytic"]

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
6 − 3z/20 at height z), with a ring groove turned into it: everything outside radius 4 from z = 8 to
z = 12 cut away. The groove has three faces, a floor (the cylinder of radius 4) and two walls (the
flat rings at z = 8, out to radius 4.8, and at z = 12, out to radius 4.2).

Then edit the frustum directly: delete the groove's floor and both its walls together. Deleting
faces takes them out of the solid and closes the gap they leave with the faces round them, each run
on along its own surface until they meet. The faces round the groove are the frustum's cone below
it and above it, which are one surface, so that cone runs on across the groove and the two become
one face again.

The result is the plain frustum, with three faces.

**As a set:**

$$\{\,\sqrt{x^2 + y^2} \le 6 - \tfrac{3z}{20},\ 0 \le z \le 20\,\}$$
