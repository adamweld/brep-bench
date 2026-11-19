+++
title = "A drilled plate grown by its hole's radius"
tags = ["offset", "grow", "analytic", "torus", "vanishing-face", "degenerate"]
origin = "brep-conformance 1.0.0, modify/offset-drilled-block-hole-fills"

[volume]
value = 6300.904006357122
exact = "V₀ − 2·2π(dρ²/2 − ρ/2·(ρq + d²·asin(ρ/d)) + (d³ − q³)/3), V₀ = abc + 2(ab + bc + ca)d + π(a + b + c)d² + 4πd³/3, a = b = 20, c = 4, ρ = 3, d = 3, q = √(d² − ρ²) = 0"
tex = 'V_0 - 2\cdot 2\pi\left(\tfrac{d\rho^2}{2} - \tfrac{\rho}{2}\left(\rho q + d^2\arcsin\tfrac{\rho}{d}\right) + \tfrac{d^3 - q^3}{3}\right),\quad V_0 = abc + 2(ab + bc + ca)\,d + \pi(a + b + c)\,d^2 + \tfrac{4}{3}\pi d^3,\quad a = b = 20,\ c = 4,\ \rho = d = 3,\ q = 0'

[area]
value = 2070.4846720026976
exact = "A₀ − 2πρ² + 2·2πd(ρφ − d(1 − cos φ)), A₀ = 2(ab + bc + ca) + 2π(a + b + c)d + 4πd², φ = asin(ρ/d) = π/2, a = b = 20, c = 4, ρ = d = 3"
tex = 'A_0 - 2\pi\rho^2 + 2\cdot 2\pi d\left(\rho\varphi - d(1 - \cos\varphi)\right),\quad A_0 = 2(ab + bc + ca) + 2\pi(a + b + c)\,d + 4\pi d^2,\quad \varphi = \arcsin\tfrac{\rho}{d} = \tfrac{\pi}{2},\ a = b = 20,\ c = 4,\ \rho = d = 3'
+++

The plate of `offset/drilled-plate-grown`, from (−10, −10, 0) to (10, 10, 4) with a hole of
radius 3 drilled through it along the z axis, grown by 3, exactly the hole's radius: the solid of
every point within 3 of the drilled plate.

The plate grows as a box does: each face moves 3 outward, each outer edge becomes a quarter of a
cylinder of radius 3, each corner an eighth of a ball. The hole's wall moves 3 in, onto the z axis,
so the hole closes: every point of it is within 3 of its wall, and it is filled. Over each end of
the hole, though, a point is within 3 of the plate only within 3 of the hole's rim, so the grown
top (and bottom) is not flat there but dips in a dimple: the part of a horn torus, a tube of
radius 3 about the rim's circle of radius 3, inside the rim. Its surface comes down to a point on
the axis, at z = 4 (and z = 0), where it meets itself in a cusp.

**As a set:**

$$\bigl(\bigl([-10,10]^2\times[0,4]\bigr) \setminus \{\,\rho \le 3\,\}\bigr) \oplus B_{3}$$
