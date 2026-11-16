+++
title = "A turned post with its shoulder rounded, shrunk by 2, past the round"
tags = ["offset", "shrink", "analytic", "torus", "cone", "round", "vanishing-face"]

[volume]
value = 130.55062804917586
exact = "2π∫∫ρ dA over the inset half-section (Pappus): the polygon's first moment about the axis, Σ(ρᵢ + ρᵢ₊₁)(ρᵢzᵢ₊₁ − ρᵢ₊₁zᵢ)/6, with the corners in the text"

[area]
value = 136.1356816555577
exact = "Σ π(ρᵢ + ρᵢ₊₁)·|segment| over the inset half-section's sides off the axis"
+++

A turned post on the z axis: the cylinder of radius 5 from z = 0 to z = 6, topped by the cone frustum from radius 5 at
z = 6 to radius 2 at z = 10 (its side the cone 4r + 3z = 38), joined. The convex edge between the cylinder and the cone,
the circle of radius 5 at z = 6, is rounded to radius 1: a piece of a torus tangent to both. Shrink it by 2: the solid
of every point at least 2 from its outside.

That is more than the round's radius. The round moved 2 inward would be a torus of tube radius 1 − 2 < 0, turned
inside out, and drops; the moved cylinder and cone meet in a sharp edge. The post is convex, and the rounded post lies
between the sharp post shrunk by 1 and grown back by 1 and the sharp post itself; both shrink by 2 to the same solid,
the sharp post's points at least 2 inside: its half-section the polygon (0, 2), (3, 2), (3, 5.33333333333),
(1, 8), (0, 8) turned about the axis (the base up 2, the side in 2, the cone in 2 along its
normal, 4r + 3z = 38 − 5·2, the top down 2).

**As a set:**

$$\{\, d \le z \le 10 - d,\ \sqrt{x^2 + y^2} \le 5 - d,\ 4\sqrt{x^2 + y^2} + 3z \le 38 - 5d \,\},\quad d = 2$$
