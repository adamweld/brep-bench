+++
title = "A cube less a post along one of its edges"
tags = ["tangent", "boolean", "cut", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/bopcut_simple/Z9"

[volume]
value = 803.65045915063792
exact = "10³ − (π5²/4)·10 = 1000 − 125π/2"
tex = '10^3 - \tfrac{\pi 5^2}{4}\cdot 10 = 1000 - \tfrac{125\pi}{2}'

[area]
value = 539.26990816987242
exact = "the cube's 600, less the quarter disc on the top and on the base 2·25π/4, less the strip 5·10 each face x = 0 and y = 0 loses, 2·50, plus the quarter-round wall (2π·5/4)·10 = 25π: 500 + 25π/2"
tex = '600 - 2\cdot\tfrac{25\pi}{4} - 2\cdot 50 + \tfrac{2\pi\cdot 5}{4}\cdot 10 = 500 + \tfrac{25\pi}{2}'
+++

A cube of side 10 between the origin and (10, 10, 10). A post: a solid cylinder of radius 5 whose axis is the line x = 0, y = 0, from z = −2 to z = 10. Its axis lies along the cube's vertical edge at x = 0, y = 0, and the post is longer than the cube: its top disc is level with the cube's top face, and it reaches 2 below the cube's base. Cut the post away from the cube.

The post's radius is half the cube's edge, so a quarter of the post lies inside the cube: a quarter-round column, taken out of the cube's whole height along that edge, reaching 5 along each of the cube faces x = 0 and y = 0.

**As a set:**

$$[0,10]^3 \setminus \{\,x^2 + y^2 \le 5^2\,\}$$
