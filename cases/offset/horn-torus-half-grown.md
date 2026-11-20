+++
title = "The outer half of a horn torus grown by 1"
tags = ["offset", "grow", "torus", "cylinder", "analytic"]
origin = "OCCT tests/offset/shape_type_i/B8"

[volume]
value = 343.99974518931086
exact = "2π∫ρ dA over the half-disk grown by 1 (Pappus): the half-disk 4π + 16/3, the strip 1 wide on its flat side 6, the half-annulus from radius 2 to 3 about its center 5π + 38/3, and two quarter-disks of radius 1 at its corners 2·(π/2 − 1/3), in all 10π + 70/3, so V = 20π² + 140π/3"
tex = 'V = 2\pi\left(10\pi + \tfrac{70}{3}\right) = 20\pi^2 + \tfrac{140\pi}{3}'

[area]
value = 283.57737656102147
exact = "2π∫ρ ds along the grown section's boundary (Pappus): the arc of radius 3, 6π + 18; the flat side's strip, 4 long at radius 1, 4; and two quarter-arcs of radius 1, 2·(π − 1): A = 2π(8π + 20) = 16π² + 40π"
tex = 'A = 2\pi\,(8\pi + 20) = 16\pi^2 + 40\pi'
+++

The half of a horn torus that lies outside the cylinder through its tube's center: the torus about
the z axis whose tube, of radius 2, runs round the circle of radius 2 in the plane z = 0, so that
it touches the axis at the origin, cut by the cylinder ρ = 2, keeping the part with ρ ≥ 2. Its
section in the half-plane through the axis is the half-disk of radius 2 about (ρ, z) = (2, 0) with
ρ ≥ 2. Its faces are the outer half of the torus and the cylinder, of radius 2 and height 4 from
z = −2 to z = 2, which meet along two circles of radius 2 at right angles. Grow it by 1: the solid
of every point within 1 of it.

The torus moves out to the torus of the same circle with a tube of radius 3, the cylinder moves
1 out of the solid, in to radius 1, and each circle where they meet becomes a quarter of a torus's
tube of radius 1 about it, joining the two. The growth stays clear of the z axis.

**As a set:**

$$\{\,(\rho - 2)^2 + z^2 \le 2^2,\ \rho \ge 2\,\} \oplus B_{1}$$
