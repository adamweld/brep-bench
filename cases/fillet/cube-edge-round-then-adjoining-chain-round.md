+++
title = "A cube's edge rounded, then the chain of edges beside the round rounded"
tags = ["fillet", "sequential", "torus", "analytic"]
origin = "OCCT tests/blend/simple/Q7"

[volume]
value = 965.225353646999
exact = "1000 − 10(9 − 9π/4) [the first round] − 14(4 − π) [the two straight runs of the second] − (π/2)(16/3 − π) [its turn about the first round's axis, Pappus] = 854 + 203π/6 + π²/2"
tex = '1000 - 10\left(9 - \tfrac{9\pi}{4}\right) - 14(4 - \pi) - \tfrac{\pi}{2}\left(\tfrac{16}{3} - \pi\right) = 854 + \tfrac{203\pi}{6} + \tfrac{\pi^2}{2}'

[area]
value = 565.036563442213
exact = "after the first round, 522 + 39π/2 (600 − 60 − 2(9 − 9π/4) + 15π); the second takes from it 64 + 3π (the strips of the top, the side, the cylinder and the face y = 0 beside it, and the two end corners 2(4 − π)) and adds its own surface, 14π + (π/2)(π + 4) = 16π + π²/2: 458 + 65π/2 + π²/2"
tex = '522 + \tfrac{39\pi}{2} - (64 + 3\pi) + 16\pi + \tfrac{\pi^2}{2} = 458 + \tfrac{65\pi}{2} + \tfrac{\pi^2}{2}'
+++

A cube of side 10 between the origin and (10, 10, 10). Two rounds, one after the other.

First round the edge where the faces x = 10 and z = 10 meet, the line from (10, 0, 10) to (10, 10, 10), to radius 3. The round is a quarter of the cylinder of radius 3 about the line x = 7, z = 7, tangent to the top face along the line x = 7 and to the face x = 10 along the line z = 7.

Then, as a second round, round to radius 2 the three edges where the face y = 0 meets the top, the first round and the face x = 10: a chain, each edge tangent to the next, the line from (0, 0, 10) to (7, 0, 10), the quarter circle of radius 3 about (7, 0, 7) from there to (10, 0, 7), and the line down to (10, 0, 0). The second round is the ball of radius 2 rolling along the chain, touching the face y = 0 and whichever of the top, the first round and the face x = 10 is beside it. Its section is a corner 2 by 2 less a quarter disk of radius 2, all along the chain; it ends square on the faces x = 0 and z = 0. Along the quarter circle it is a quarter of a torus whose axis is the first round's axis.

**As a set:** with $t = 3 - \sqrt{(x-7)^2 + (z-7)^2}$ where $x > 7$ and $z > 7$, and $t = \min(10 - x,\, 10 - z)$ elsewhere,

$$[0,10]^3 \setminus \{\, x > 7,\ z > 7,\ (x-7)^2 + (z-7)^2 > 3^2 \,\} \setminus \{\, y < 2,\ 0 \le t < 2,\ (2 - y)^2 + (2 - t)^2 > 2^2 \,\}$$
