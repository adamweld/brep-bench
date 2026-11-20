+++
title = "A band of a sphere extruded straight along the sphere's axis"
tags = ["sweep", "extrude", "analytic", "sphere", "cylinder"]
origin = "OCCT bugs/modalg_7/bug30346_2"

[volume]
value = 56.548667764616276
exact = "the band lies over the ring 4 ≤ ρ ≤ 5, and the solid is 2 thick in z over it: 2·π(5² − 4²)"
tex = '2\cdot\pi\,(5^2 - 4^2)'

[area]
value = 301.59289474462014
exact = "the band and its copy, 2·2π·5·3 = 60π, and the two straight walls, 2π·4·2 + 2π·5·2 = 36π"
tex = '2\cdot 2\pi\cdot 5\cdot 3 + 2\pi\cdot 4\cdot 2 + 2\pi\cdot 5\cdot 2'
+++

The band of the sphere of radius 5 about the origin that lies between the equator, z = 0, and the circle at height z = 3 (where the sphere's radius across is 4), in the upper half, all the way round. Extrude it straight up the z axis by 2: the solid of everything the band passes through as it is carried up 2 without turning.

The solid lies between the band and its copy 2 higher, over the ring $4 \le \rho \le 5$; its other faces are the cylinders $\rho = 4$ (from z = 3 to 5) and $\rho = 5$ (from z = 0 to 2), each 2 high.

**As a set:**

$$\{\,4 \le \rho \le 5,\ \sqrt{25 - \rho^2} \le z \le \sqrt{25 - \rho^2} + 2\,\}$$
