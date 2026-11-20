+++
title = "A sphere cap with its rim rounded"
tags = ["fillet", "sphere", "torus", "planar", "analytic"]
origin = "OCCT tests/blend/simple/J2"

[volume]
value = 47.3569106407326
exact = "2π·M by Pappus, M = ∫ρ dA over the section (the cap's section less the rim sliver), the rim round's circle of radius 1/2 about (ρ, z) = (2√2, 7/2)"

[area]
value = 84.70788744327063
exact = "2π·Σ ∫ρ ds over the section's pieces: the base line from ρ = 0 to 2√2, the arc of radius 1/2 turned from −90° to atan2(7/2, 2√2) ≈ 51.05°, and the sphere's arc from there to the pole"
+++

The cap cut off the ball of radius 5 about the origin by the plane z = 3: the part with z ≥ 3. Its
flat base is the disc of radius 4 at z = 3, and its pole is (0, 0, 5). The base meets the sphere in
a circle of radius 4, at an acute angle of 53.13°. Round that circular edge to radius 1/2.

The round is the torus a ball of radius 1/2 sweeps as it rolls around the rim, touching the flat
base and the sphere from inside. Its center runs round the circle of radius 2√2 at z = 7/2, where
the ball is 4.5 from the origin and 1/2 above the base. It touches the base along the circle of
radius 2√2 and the sphere along the circle 5/4.5 times as far out from the axis and from the
origin as its center.

**As a set:**

$$\bigl((B_5 \cap \{\,z \ge 3\,\}) \ominus B_{1/2}\bigr) \oplus B_{1/2}$$

the cap shrunk by 1/2 (B₄.₅ ∩ { z ≥ 7/2 }) and grown by 1/2 again.
