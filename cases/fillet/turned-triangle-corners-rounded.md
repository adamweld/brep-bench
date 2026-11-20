+++
title = "A triangular ring turned three quarters of a turn, its three long edges rounded"
tags = ["fillet", "torus", "cylinder", "cone", "planar", "revolve", "analytic"]
origin = "OCCT tests/blend/simple/I1, OCCT tests/blend/simple/I2, OCCT tests/blend/simple/I3, OCCT tests/blend/simple/I4, OCCT tests/blend/simple/H7, OCCT tests/blend/simple/H9, OCCT tests/blend/simple/J1"

[volume]
value = 437.27095883219315
exact = "(3π/2)·M by Pappus, M = ∫ρ dA over the rounded section = 120 − Σᵢ (kiteᵢ − sectorᵢ) moments = 92.79177942490162; kiteᵢ = the two triangles (V, T₁, C), (V, C, T₂) of area r·tᵢ/2 each, sectorᵢ = the arc's sector of area (π − θᵢ)r²/2 with its centroid 4r·sin((π − θᵢ)/2)/(3(π − θᵢ)) from C towards V; r = 1, tᵢ = r/tan(θᵢ/2) = (3 + √13)/2, 1, (2 + √13)/3, θᵢ = atan(2/3), π/2, atan(3/2)"

[area]
value = 560.6354237316693
exact = "(3π/2)·Σ ∫ρ ds over the section's three straight pieces and three arcs, plus the two flat ends, each the rounded section's area 12 + π − (19 + 5√13)/6 = 8.970299924036469"
+++

A right triangle in the half-plane y = 0, x ≥ 0 (ρ = x), with corners at (ρ, z) = (6, 0), (12, 0) and
(12, 4), turned about the z axis from φ = 0° to φ = 270°, right-handed. The solid is a
three-quarter ring with a triangular section: its underside a flat annulus at z = 0 from ρ = 6 to
ρ = 12; its outer wall the cylinder of radius 12, from z = 0 to z = 4; its sloping inside the cone
from radius 6 at z = 0 to radius 12 at z = 4; and two flat triangular ends in the half-planes
φ = 0° and φ = 270°.

Round its three long edges, each three quarters of a circle: the edge where the underside meets
the cone (radius 6, at an acute angle of atan(2/3) in the section), where the underside meets the
wall (radius 12), and where the wall meets the cone (radius 12, at z = 4), all to radius 1. Each
round is the torus a circle of radius 1 sweeps turning with the edge, tangent to the two faces it
joins, and runs the whole length of its edge, ending in the flat end faces, whose three corners it
rounds the same way. The three rounds do not meet one another. In the section, each corner of the
triangle becomes an arc of radius 1 about (ρ, z) = ((15 + √13)/2, 1), (11, 1) and
(11, 4 − (2 + √13)/3), turned through π − atan(2/3), π/2 and π − atan(3/2).

**As a set:**

$$\{\,(\rho, \varphi, z) : (\rho, z) \in (T \ominus B_1) \oplus B_1,\ 0^\circ \le \varphi \le 270^\circ\,\}$$

where T is the triangle with corners (6, 0), (12, 0), (12, 4), and the opening is taken in the (ρ, z)
plane.
