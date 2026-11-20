+++
title = "A half torus with the outer edge of its flat face rounded"
tags = ["fillet", "torus", "planar", "analytic"]
origin = "OCCT tests/blend/simple/J3, OCCT tests/blend/simple/J4, OCCT tests/blend/simple/J5"

[volume]
value = 193.63033907538149
exact = "2π·M by Pappus, M = ∫ρ dA over the section: the half disc of radius 2 about (5, 0), less the corner sliver at (7, 0) that the arc of radius 1/2 about (5 + √2, 1/2) rounds"

[area]
value = 309.23469359754405
exact = "2π·Σ ∫ρ ds over the section's pieces: the flat from ρ = 3 to 5 + √2, the arc of radius 1/2 from −90° to asin(1/3), and the tube's arc of radius 2 about (5, 0) from asin(1/3) to 180°"
+++

The upper half of a solid torus: the part with z ≥ 0 of the solid whose tube, of radius 2, runs
round the circle of radius 5 about the z axis in the plane z = 0. Its flat face is the annulus
3 ≤ ρ ≤ 7 at z = 0, and it meets the torus at a right angle along the two circles ρ = 3 and ρ = 7.
Round the outer one, the circle of radius 7, to radius 1/2; leave the inner edge sharp.

The round is the surface a ball of radius 1/2 sweeps rolling round that edge, touching the flat and
the torus from inside. Its center runs round the circle of radius 5 + √2 at z = 1/2 (the ball is
1/2 above the flat and 3/2 from the tube's core circle). It touches the flat along that circle,
and the torus along the circle where the line from the tube's center (5, 0) through the ball's
center, at an angle asin(1/3) above the flat, meets the tube.
