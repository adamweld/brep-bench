+++
title = "A cylinder less a ball and a slab through it"
tags = ["boolean", "cut", "fuse", "sphere", "cylinder", "multi-solid", "analytic"]
origin = "OCCT tests/bugs/modalg_7/bug24692"

[volume]
value = 309.9704751541929
exact = "2·2π∫₀³ ρ(10 − √(25 − ρ²)) dρ = 2·2π(45 − 61/3) = 296π/3: each end of the cylinder, down to or up from the ball's surface"
tex = '2\cdot 2\pi\int_0^3 \rho\left(10 - \sqrt{25 - \rho^2}\right)d\rho = \tfrac{296\pi}{3}'

[area]
value = 345.57519189487726
exact = "2(9π + 2π·3·6 + 2π·5·1) = 110π: for each of the two pieces its flat end, its wall of height 6 and the ball's zone of height 1 (from z = 5 to 6, or 14 to 15)"
tex = '2\left(9\pi + 2\pi\cdot 3\cdot 6 + 2\pi\cdot 5\cdot 1\right) = 110\pi'
+++

Two bodies, each a fuse.

The first is a cylinder of radius 3 about the z axis from z = 0 to z = 20, fused with a flat
square slab through it: the box from (−6, −6, 9) to (6, 6, 11).

The second is a ball of radius 5 centred at (0, 0, 10), fused with the same slab.

Cut the second from the first. The slab and the ball between them take away the whole middle of
the cylinder, from the ball's lower surface to its upper one, and what is left is two separate
solids: the foot of the cylinder under the ball, and its head above it, each with a spherical
dish for one face. The ball meets the cylinder's wall in circles of radius 3 at z = 6 and at
z = 14, the spherical edges of the two pieces.

The volume and the area are those of the two pieces taken together.

**As a set:**

$$\{\,\rho \le 3,\ 0 \le z \le 20\,\} \setminus \bigl(B_5(0, 0, 10) \cup [-6, 6]^2 \times [9, 11]\bigr)$$
