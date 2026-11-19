+++
title = "A drilled plate grown past its hole"
tags = ["offset", "grow", "analytic", "torus", "vanishing-face", "self-intersection"]
origin = "brep-conformance 1.0.0, modify/offset-drilled-block-hole-fills"

[volume]
value = 8548.44724650172
exact = "V₀ − 2·2π(dρ²/2 − ρ/2·(ρq + d²·asin(ρ/d)) + (d³ − q³)/3), V₀ = abc + 2(ab + bc + ca)d + π(a + b + c)d² + 4πd³/3, a = b = 20, c = 4, ρ = 3, d = 4, q = √(d² − ρ²) = √7"
tex = 'V_0 - 2\cdot 2\pi\left(\tfrac{d\rho^2}{2} - \tfrac{\rho}{2}\left(\rho q + d^2\arcsin\tfrac{\rho}{d}\right) + \tfrac{d^3 - q^3}{3}\right),\quad V_0 = abc + 2(ab + bc + ca)\,d + \pi(a + b + c)\,d^2 + \tfrac{4}{3}\pi d^3,\quad a = b = 20,\ c = 4,\ \rho = 3,\ d = 4,\ q = \sqrt{7}'

[area]
value = 2430.166661073631
exact = "A₀ − 2πρ² + 2·2πd(ρφ − d(1 − cos φ)), A₀ = 2(ab + bc + ca) + 2π(a + b + c)d + 4πd², φ = asin(ρ/d), a = b = 20, c = 4, ρ = 3, d = 4"
tex = 'A_0 - 2\pi\rho^2 + 2\cdot 2\pi d\left(\rho\varphi - d(1 - \cos\varphi)\right),\quad A_0 = 2(ab + bc + ca) + 2\pi(a + b + c)\,d + 4\pi d^2,\quad \varphi = \arcsin\tfrac{\rho}{d},\ a = b = 20,\ c = 4,\ \rho = 3,\ d = 4'
+++

The plate of `offset/drilled-plate-grown`, from (−10, −10, 0) to (10, 10, 4) with a hole of
radius 3 drilled through it along the z axis, grown by 4, more than the hole's radius: the solid of
every point within 4 of the drilled plate.

The plate grows as a box does: each face moves 4 outward, each outer edge becomes a quarter of a
cylinder of radius 4, each corner an eighth of a ball. The hole is gone: every point of it is
within 3 of its wall, so it is filled. Over each end of the hole, though, a point is within 4 of
the plate only within 4 of the hole's rim, so the grown top (and bottom) dips there in a dimple:
the inner part of a spindle torus, a tube of radius 4 about the rim's circle of radius 3, which
crosses the axis. The dimple's lowest point is on the axis, √7 ≈ 2.65 above the top face (and
below the bottom), where the spindle's surface meets itself in a point, and it meets the flat of
the grown top along the circle of radius 3 over the rim.

**As a set:**

$$\bigl(\bigl([-10,10]^2\times[0,4]\bigr) \setminus \{\,\rho \le 3\,\}\bigr) \oplus B_{4}$$
