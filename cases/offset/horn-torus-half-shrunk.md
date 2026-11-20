+++
title = "The outer half of a horn torus shrunk by a half"
tags = ["offset", "shrink", "torus", "cylinder", "analytic"]
origin = "OCCT tests/offset/shape_type_i/B9"

[volume]
value = 37.766479520236795
exact = "2π∫ρ dA over the section (Pappus): the section is the part of the disk of radius 3/2 about (ρ, z) = (2, 0) beyond ρ = 5/2, whose ∫ρ dA = ∫ from 1/2 to 3/2 of (2 + x)·2√(9/4 − x²) dx = 9π/4 + √2/3 − (9/2)asin(1/3). So V = π(27π + 4√2 − 54·asin(1/3))/6"
tex = 'V = 2\pi\left(\tfrac{9\pi}{4} + \tfrac{\sqrt2}{3} - \tfrac92\arcsin\tfrac13\right)'

[area]
value = 117.49220375915326
exact = "2π∫ρ ds along the section's boundary (Pappus): the arc of radius 3/2 about (2, 0), through half-angle α = acos(1/3) each side of the ρ axis, 2π(6α + 3√2); and the cylinder of radius 5/2 and height 2√2, 10√2·π. In all 12π·acos(1/3) + 16√2·π"
tex = 'A = 12\pi\arccos\tfrac13 + 16\sqrt2\,\pi'
+++

The half of a horn torus that lies outside the cylinder through its tube's center: the part with ρ ≥ 2 of the torus about the z axis whose tube, of radius 2, runs
round the circle of radius 2 in the plane z = 0 and so touches the axis at the origin. Its section in
the half-plane through the axis is the half-disk of radius 2 about (ρ, z) = (2, 0) with ρ ≥ 2. Its
faces are the outer half of the torus and the cylinder of radius 2 and height 4, from z = −2 to
z = 2. Shrink it by 1/2: the solid of every point at least 1/2 from its outside.

The torus moves in to the torus of the same circle with a tube of radius 3/2, and the cylinder
moves 1/2 into the solid, out to radius 5/2. Where the two now meet, on a circle of radius 5/2 at
z = ±√2, the cylinder, of height 2√2, closes the torus's outer part, the arc from
−acos(1/3) to acos(1/3) about the tube's center. The result is a sliver of a torus tube cut by a
cylinder.

**As a set:**

$$\{\,(\rho - 2)^2 + z^2 \le 2^2,\ \rho \ge 2\,\} \ominus B_{1/2}$$
