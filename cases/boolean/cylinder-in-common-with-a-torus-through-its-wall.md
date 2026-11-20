+++
title = "A tall cylinder and a coaxial torus whose tube is centred on its wall, their common part"
tags = ["boolean", "common", "analytic", "torus", "cylinder"]
origin = "OCCT tests/boolean/bopcommon_simple/ZL2"

[volume]
value = 35.289627399571043
exact = "by Pappus, the half tube (a half disc of radius 1, its centroid 4/(3π) inside the tube's centre circle) swept once round: V = 2π(4 − 4/(3π))·π/2 = 4π² − 4π/3"
tex = '4\pi^2 - \tfrac{4\pi}{3}'

[area]
value = 116.65594705179239
exact = "the torus half 2π·r·(πR − 2r) = 2π(4π − 2), plus the cylinder's wall inside the tube, a band 2 high: 2π·4·2 = 16π; total 8π² + 12π"
tex = '8\pi^2 + 12\pi'
+++

A cylinder of radius 4 on the z axis, tall — from z = −5 to z = 5 — and a torus on the same axis: its tube's centre circle is the circle of radius 4 in the plane z = 0, the tube's radius 1. Take the part common to the cylinder and the torus.

The cylinder's wall runs through the middle of the tube, along the circle where the tube is widest across, so it divides the torus into an inner half and an outer half; the common part is the inner half, whose cross-section by a plane through the axis is a half disc of radius 1 with its flat side on the cylinder's wall. It is a ring with a round inner face (the half of the torus nearer the axis) and a flat outer face, the band of the cylinder's wall from z = −1 to z = 1.

**As a set:**

$$\{\rho \le 4,\ |z| \le 5\} \cap \{(\rho - 4)^2 + z^2 \le 1\}$$
