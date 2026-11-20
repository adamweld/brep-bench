+++
title = "A post whose axis lies along a cube's edge, less the cube"
tags = ["boolean", "cut", "analytic", "cylinder", "planar"]
origin = "OCCT tests/boolean/boptuc_simple/Z9"

[volume]
value = 746.12825522757589
exact = "V = (3/4)·25π·12 + (1/4)·25π·2: the post (radius 5, length 12) less the quarter of it inside the cube, 25π·10/4"
tex = '\tfrac34\cdot 25\pi\cdot 12 + \tfrac14\cdot 25\pi\cdot 2 = \tfrac{475\pi}{2}'

[area]
value = 555.53093477052002
exact = "area = 25π (the post's base disc) + (3/4)·25π (its top disc, 3 quarters) + 95π (its wall, 2π·5·12 less the quarter inside the cube, 25π) + 25π/4 (the cube's base plane where the post goes on below it) + 2·50 (the cube's faces x = 0 and y = 0, 5 by 10 each): 145π + 100"
tex = '25\pi + \tfrac{75\pi}{4} + 95\pi + \tfrac{25\pi}{4} + 100 = 145\pi + 100'
+++

A cube of side 10 between the origin and (10, 10, 10). A post: a solid cylinder of radius 5 whose axis is the line x = 0, y = 0, from z = −2 to z = 10. Its axis lies along the cube's vertical edge at x = 0, y = 0, and the post is longer than the cube: its top disc is level with the cube's top face, and it reaches 2 below the cube's base. Cut the cube away from the post.

A quarter of the post lies inside the cube, a quarter-round column along its edge. What is left of the post is three quarters of it, whole from z = −2 to z = 10, and under the cube's base, from z = −2 to z = 0, the other quarter as well. It is one piece.

**As a set:**

$$\{\,\rho \le 5,\ -2 \le z \le 10\,\} \setminus [0,10]^3$$
