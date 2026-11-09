+++
title = "A thicker torus under the nearly perpendicular tumble"
tags = ["swept", "solid-sweep", "tumble", "torus", "regression"]
origin = "geomsolver 55e64186 tumbling-torus failure isolation; docs/tumbling-torus-regressions.md"
settled = false
+++

The initial torus is centered at (2.02, 0, 1), with axis parallel to z, major radius 1 and tube
radius 0.16. The major radius is the radius of the circle through the tube's centers. It is a ring
torus, including its interior.

Let A be the oriented line through (2, 0, 0) in direction (0, 0, 1). Define ℓ = 1°, α = 89° and h =
0. Let B be the oriented line through q = (0, h cos ℓ, h sin ℓ), with unit direction u = (cos α, sin
α cos ℓ, sin α sin ℓ).

At roll t, first turn the initial torus by 1.01t about A, then turn that result by -1t about B. Both
angles use the same t; the second axis stays fixed in the world. Take the union of every occupied
position for **all real t from -180° to 180°**, including both end poses. This is the continuous
swept solid, not a finite union of sampled poses. Overlapped material is counted once; interior
envelope sheets are not part of the boundary.

A control for `swept/tumbling-torus-near-orthogonal`: only the tube radius changes, from 0.08 to
0.16.

No closed-form volume or area is asserted. The case remains unsettled until independently
implemented kernels agree under the benchmark's rules.

**As a set:**

$$\bigcup_{t \in [-180^\circ,\,180^\circ]} R_B(-t)\,R_A(1.01\,t)\,T, \quad T =
\bigl\{\,\bigl(\sqrt{(x-2.02)^2 + y^2} - 1\bigr)^2 + (z-1)^2 \le 0.16^2\,\bigr\}$$
