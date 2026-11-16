+++
title = "A rounded cone frustum hollowed with walls thicker than its rounds"
tags = ["shell", "analytic", "torus", "cone", "round", "vanishing-face"]

[volume]
value = 316.73927263788545
exact = "V_R − V_C: V_R = 126π − 2πΣM, M the first moment about the axis of the region a round cuts from the half-section (kite less sector; the bottom rim's 6.961239708142603; the top rim's 0.419417844371199); V_C = πh(a² + ab + b²)/3, a = 2.763932022500210, b = 1.763932022500210, h = 2"

[area]
value = 325.36680328498096
exact = "A_R + the cavity's faces, A_R the rounded frustum's area (the flat faces and cone less what the rounds take, plus each round's torus band, 2πr∫ρ dα)"
+++

A cone frustum on the z axis, from the disk of radius 6 at z = 0 to the disk of radius 3 at z = 6 (its side
the cone 2r + z = 12), with both rims rounded to radius 1.5, each round a piece of a torus tangent to the flat face and to
the cone. Hollow it to walls 2 thick: keep every point within 2 of its outside, leaving a closed cavity, every point at least 2 inside: the frustum from z = 2 to z = 4, its side the cone 2r + z = 12 − 2√5. The walls are thicker than the rounds' radius, so the rounds moved inward turn inside out and drop, and the cavity is the sharp frustum shrunk by the wall, as in `offset/rounded-frustum-shrunk-past-its-rounds`.

The result is the rounded frustum less that cavity. The rounded frustum's measures follow from Pappus's theorem on its
half-section, the trapezoid (0, 0), (6, 0), (3, 6), (0, 6) with the rounded corner(s) cut to arcs of radius 1.5: each arc's
corner region is the kite between the corner, the arc's two tangent points and its centre, less the circular sector, all
in closed form.
