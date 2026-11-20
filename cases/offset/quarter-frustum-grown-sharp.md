+++
title = "A quarter of a cone frustum grown by 1, its edges kept sharp"
tags = ["offset", "grow", "cone", "sharp-edge", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug427_1"

[volume]
value = 238.38586482092635
exact = "∫₋₁⁵ A(ρ(z)) dz = (4/3)(F(8) − F(7/2)), ρ(z) = 29/4 − 3z/4, A(ρ) = πρ²/4 + √(ρ² − 1) + ρ² asin(1/ρ) + 1 the area of the section, F(ρ) = πρ³/12 + (2/3)ρ√(ρ² − 1) − (1/3)ln(ρ + √(ρ² − 1)) + (ρ³/3) asin(1/ρ) + ρ"
tex = '\int_{-1}^{5} A(\rho(z))\,dz = \tfrac43\bigl(F(8) - F(\tfrac72)\bigr)'

[area]
value = 247.46660502152588
exact = "A(7/2) + A(8) for the top and bottom, (5/3)∫ ρ(π/2 + 2 asin(1/ρ)) dρ over ρ from 7/2 to 8 for the cone's wall, (8/3)∫ (√(ρ² − 1) + 1) dρ over the same for the two flat sides, A as in the volume"
tex = 'A(\tfrac72) + A(8) + \tfrac53\int_{7/2}^{8}\rho\left(\tfrac\pi2 + 2\arcsin\tfrac1\rho\right)d\rho + \tfrac83\int_{7/2}^{8}\left(\sqrt{\rho^2 - 1} + 1\right)d\rho'
+++

A quarter of a cone frustum: the part of the frustum with radius 6 at z = 0 and radius 3 at z = 4,
about the z axis, whose angle φ about the z axis, measured from +x toward +y, lies between 0° and
90°. Its faces are the bottom and the top (a quarter disk each), the cone's wall, and two flat
sides, in the planes x = 0 and y = 0, that meet along the z axis. Grow it by 1 with its edges kept
sharp: move each face 1 outward along its normal and extend the moved faces until they meet, so
that no round is added.

The bottom moves to z = −1 and the top to z = 5; the plane x = 0 to x = −1 and the plane y = 0 to
y = −1; the wall to the cone ρ = 29/4 − 3z/4, the cone ρ = 6 − 3z/4 moved 1 outward square to its
slant, which is 5/4 in ρ. The grown solid is bounded by these five and is convex; its radius is 8 at
z = −1 and 3.5 at z = 5, and the corner of the two flat sides, (−1, −1), lies inside the cone at
every height.

**As a set:**

$$\{\,x \ge -1,\ y \ge -1,\ -1 \le z \le 5,\ \rho + \tfrac34 z \le \tfrac{29}{4}\,\}$$
