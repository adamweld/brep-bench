+++
title = "An oblique torus tumble through half a turn"
tags = ["swept", "solid-sweep", "tumble", "torus", "regression"]
origin = "geomsolver 55e64186 tumbling-torus failure isolation; docs/tumbling-torus-regressions.md"
settled = false
+++

The initial torus is centered at (3, 0, 2), with axis parallel to z, major radius 1 and tube radius
0.16. The major radius is the radius of the circle through the tube's centers. It is a ring torus,
including its interior.

Let A be the oriented line through (2, 0, 0) in direction (0, 0, 1). Define ℓ = 30°, α = 20° and h =
0.7071. Let B be the oriented line through q = (0, h cos ℓ, h sin ℓ), with unit direction u = (cos
α, sin α cos ℓ, sin α sin ℓ).

At roll t, first turn the initial torus by 0.375t about A, then turn that result by -1t about B.
Both angles use the same t; the second axis stays fixed in the world. Take the union of every
occupied position for **all real t from -90° to 90°**, including both end poses. This is the
continuous swept solid, not a finite union of sampled poses. Overlapped material is counted once;
interior envelope sheets are not part of the boundary.

A shorter-interval control for `swept/tumbling-torus-full-turn`. The roll spans 180 degrees instead
of 360 degrees; no other parameter changes.

No closed-form volume or area is asserted. The case remains unsettled until independently
implemented kernels agree under the benchmark's rules.
