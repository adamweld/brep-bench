+++
title = "A bipyramid on a triangle"
tags = ["loft", "ruled", "point-ends", "pyramid", "analytic"]
origin = "OCCT thrusection/specific/A1"

[volume]
value = 62.353829072479583
exact = "(1/3)·A·(6 + 3), A = 12√3 the triangle's area"
tex = '\tfrac13\cdot 12\sqrt3\cdot(6+3) = 36\sqrt3'

[area]
value = 103.19669489101032
exact = "six flat triangles: each of the upper three has base 4√3 and height √(6² + 2²) = √40, each of the lower three height √(3² + 2²) = √13; the 2 is the triangle's inradius"
tex = '3\cdot\tfrac12\cdot4\sqrt3\,(\sqrt{40}+\sqrt{13}) = 12\sqrt{30}+6\sqrt{39}'
+++

Two three-sided pyramids joined base to base, as a ruled loft from a point through a triangle to a second point, the two points over the middle of the triangle and at different heights.

The triangle lies in the plane z = 0, with corners (4, 0, 0), (−2, 2√3, 0) and (−2, −2√3, 0): equilateral, side 4√3, centered on the z axis. The two points are (0, 0, 6) above it and (0, 0, −3) below it. Each point is joined to every point of the triangle's boundary by a straight line, so the solid has six flat faces: three triangles rising to the upper point and three falling to the lower one.

**As a set:**

$$\operatorname{conv}\bigl((4,0,0),\ (-2,\pm2\sqrt3,0),\ (0,0,6),\ (0,0,-3)\bigr)$$
