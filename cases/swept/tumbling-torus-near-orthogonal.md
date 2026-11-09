+++
title = "A thin torus tumbled about nearly perpendicular axes"
tags = ["swept", "solid-sweep", "tumble", "torus", "regression"]
origin = "geomsolver 55e64186 tumbling-torus failure isolation; docs/tumbling-torus-regressions.md"
settled = false
+++

The initial torus is centered at (2.02, 0, 1), with axis parallel to z, major radius 1 and tube
radius 0.08. The major radius is the radius of the circle through the tube's centers. It is a ring
torus, including its interior.

Let A be the oriented line through (2, 0, 0) in direction (0, 0, 1). Define ℓ = 1°, α = 89° and h =
0. Let B be the oriented line through q = (0, h cos ℓ, h sin ℓ), with unit direction u = (cos α, sin
α cos ℓ, sin α sin ℓ).

At roll t, first turn the initial torus by 1.01t about A, then turn that result by -1t about B. Both
angles use the same t; the second axis stays fixed in the world. Take the union of every occupied
position for **all real t from -180° to 180°**, including both end poses. This is the continuous
swept solid, not a finite union of sampled poses. Overlapped material is counted once; interior
envelope sheets are not part of the boundary.

Almost perpendicular axes, nearly equal turn ratios and a thin tube make a narrow swept region.
Compare the identical motion with twice the tube radius in
`swept/tumbling-torus-near-orthogonal-thick`.

No closed-form volume or area is asserted. The case remains unsettled until independently
implemented kernels agree under the benchmark's rules.

**As a set:**

$$\bigcup_{t \in [-180^\circ,\,180^\circ]} R_B(-t)\,R_A(1.01\,t)\,T, \quad T =
\bigl\{\,\bigl(\sqrt{(x-2.02)^2 + y^2} - 1\bigr)^2 + (z-1)^2 \le 0.08^2\,\bigr\}$$
