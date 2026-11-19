+++
title = "A cube with a smaller cube cut from its corner"
tags = ["boolean", "cut", "analytic", "coincident", "box"]
origin = "OCCT fclasses/bug7287_1"

[volume]
value = 936.0
exact = "10³ − 4³"
tex = '10^3 - 4^3'

[area]
value = 600.0
exact = "6·10² − 3·4² + 3·4²"
tex = '6\cdot 10^2 - 3\cdot 4^2 + 3\cdot 4^2'
+++

A cube of side 10 from the origin to (10, 10, 10), and a cube of side 4 from the origin to (4, 4, 4). The small cube shares the large one's corner at the origin and lies against its three faces there. Cut the small cube from the large one, leaving a cube with a square notch at one corner.

**As a set:**

$$[0,10]^3\setminus[0,4]^3$$
