+++
title = "A plate pocketed between a tilted roof and a V-shaped floor"
tags = ["feature", "pocket", "up-to-face", "planar", "analytic"]
origin = "OCCT feat/featprism/F8, OCCT feat/featprism/F9, OCCT feat/featprism/G3, OCCT feat/featprism/G8"

[volume]
value = 6816.6666666666667
exact = "40·30·6 − 10·∫₁₅²⁵ (min(8 − 0.3(x − 15), 6) − (1.5 + 0.2|x − 20|)) dx = 7200 − 10·(115/3) = 7200 − 1150/3"
tex = '40\cdot 30\cdot 6 - 10\int_{15}^{25}\left[\min\bigl(8 - 0.3(x - 15),\,6\bigr) - 1.5 - 0.2\,|x - 20|\right]dx = 7200 - \tfrac{1150}{3}'

[area]
value = 3446.7814119682242
exact = "plate 3240, less the top the pocket opens in 10·(20/3), plus the roof (100/3)·√1.09, the floor 100·√1.04, the end walls 35 + 25 and the side walls 2·(115/3): = 3310 + (10/3)√109 + 20√26"
tex = '3310 + \tfrac{10}{3}\sqrt{109} + 20\sqrt{26}'
+++

A thin plate with a pocket cut into it, bounded above by a tilted plane and below by two planes
that meet in a valley: the pocket's ceiling and floor are given faces, neither flat nor level.

- The plate: 0 ≤ x ≤ 40, 0 ≤ y ≤ 30, 0 ≤ z ≤ 6.
- The pocket: everything over the square 15 ≤ x ≤ 25, 10 ≤ y ≤ 20 that lies between two faces.
  The *roof* is the plane z = 8 − 0.3·(x − 15): above the plate's top, z = 6, up to x = 21⅔, and
  below it from there to x = 25, where it is at z = 5. The *floor* is the two planes
  z = 1.5 + 0.2·|x − 20|, meeting along the valley line x = 20, z = 1.5 and rising to z = 2.5 at
  x = 15 and at x = 25.
- Cut away the part of the plate between the roof and the floor, inside the square's four vertical
  walls.

So the pocket opens through the plate's top from x = 15 to x = 21⅔; beyond that the roof is an
overhang, a lid of plate between the roof and the plate's top (up to 1 thick), and the pocket ends
in a wall at x = 25, 2.5 high. At x = 15 its wall is 3.5 high.

**As a set:**

$$\bigl([0,40]\times[0,30]\times[0,6]\bigr) \setminus \{\,15 \le x \le 25,\ 10 \le y \le 20,\ 1.5 + 0.2\,|x - 20| \le z \le 8 - 0.3\,(x - 15)\,\}$$
