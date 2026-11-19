+++
title = "A cube notched by a half cone that reaches two of its corners"
tags = ["tangent", "boolean", "cut", "analytic", "cone", "point-contact"]
origin = "OCCT tests/boolean/bopcut_simple/ZG5"

[volume]
value = 795.7964775166635
exact = "a³ − ½·πh(R₁² + R₁R₂ + R₂²)/3 = 1000 − 65π, a = h = 10, R₁ = 5, R₂ = 2: the half of the frustum inside the cube"
tex = 'a^3 - \tfrac12\cdot\tfrac{\pi h}{3}\left(R_1^2 + R_1 R_2 + R_2^2\right) = 1000 - 65\pi,\quad a = h = 10,\ R_1 = 5,\ R_2 = 2'

[area]
value = 599.2440723266147
exact = "6a² − h(R₁ + R₂) − π(R₁² + R₂²)/2 + π(R₁ + R₂)s/2 = 530 − 29π/2 + 7√109·π/2, s = √(h² + (R₁ − R₂)²) = √109 the slant: the face x = 10 loses the frustum's section through its axis, the bottom and the top each a half disk, and half the cone's side is added"
tex = '6a^2 - h(R_1 + R_2) - \tfrac{\pi}{2}\left(R_1^2 + R_2^2\right) + \tfrac{\pi}{2}(R_1 + R_2)s = 530 - \tfrac{29\pi}{2} + \tfrac{7\sqrt{109}}{2}\pi,\quad s = \sqrt{109}'
+++

A cube from (0, 0, 0) to (10, 10, 10), less a solid cone frustum whose axis is the upright line
x = 10, y = 5, in the cube's face x = 10: its base the disk of radius 5 at z = 0, and its top the
disk of radius 2 at z = 10.

Half the frustum is inside the cube. Its base circle, whose radius is half the face's width,
touches the cube's faces y = 0 and y = 10 at the cube's corners (10, 0, 0) and (10, 10, 0); above
the base the cone narrows and stays clear of both faces. The cut leaves a notch of half-cone shape
down the face x = 10, as wide as the face at the bottom and 4 wide at the top. The face x = 10 is
left as two triangles, each with its tip at one of those two corners. The bottom face is left as
the square less a half disk whose rim runs into each of those corners tangent to the cube's edge,
so that the face narrows to nothing there. The faces y = 0 and y = 10 stay whole: the cone touches
each only at the corner.

**As a set:**

$$[0,10]^3 \setminus \{\,(x - 10)^2 + (y - 5)^2 \le \left(5 - \tfrac{3z}{10}\right)^2,\ 0 \le z \le 10\,\}$$
