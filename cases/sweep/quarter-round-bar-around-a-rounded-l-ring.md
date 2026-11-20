+++
title = "A bar of quarter-round section swept around a rounded L-shaped ring"
tags = ["sweep", "planar-path", "closed", "arc", "concave", "analytic"]
origin = "OCCT tests/evolved/voluved/AGT008"

[volume]
value = 4393.781261219953
exact = "A(P + 2πs̄): the section's area A = 9π, the path's length P = 160 − 6·8(2 − π/2) = 64 + 24π, and its centroid's distance outward s̄ = 4·6/(3π) = 8/π; the path turns through 2π in all, and a point s outward from it goes 2πs further for that: = 9π(80 + 24π)"
tex = 'A\left(P + 2\pi\bar{s}\right),\quad A = 9\pi,\ P = 64 + 24\pi,\ \bar{s} = \tfrac{8}{\pi}:\ = 9\pi(80 + 24\pi)'

[area]
value = 3325.867997189232
exact = "Σ over the section's edges e of ∫(P + 2πs) dℓ: the flat 6(P + 6π), the upright 6P, and the arc 3πP + 72π: = (12 + 3π)P + 108π = 768 + 588π + 72π²"
tex = '\sum_e \int_e (P + 2\pi s)\,d\ell = (12 + 3\pi)P + 108\pi = 768 + 588\pi + 72\pi^2'
+++

A bar swept around the outside of an L-shaped outline whose corners are all rounded.

The path: the outline of the L in the xy plane with corners (0, 0), (40, 0), (40, 20), (20, 20),
(20, 40) and (0, 40), run in that order, counterclockwise, with each of its six corners rounded by an
arc of radius 8 tangent to both its sides. The five convex corners are rounded off the L's inside,
and the concave one, at (20, 20), the other way, into the notch. The path is closed, 64 + 24π ≈ 139.4
long, and smooth.

The section stands square to the path. It is given by s, the distance outward from the path (to its
right as it is run, away from the L), and z: the quarter of the disk of radius 6 about the origin
that has s ≥ 0 and z ≥ 0. That is a bar with a flat bottom, an upright inner face along the path, and
a quarter-round outer face. On the side y = 0, for example, the point (s, z) of the section is at
(x, −s, z).

The bar is that section carried once around the path, with no ends, turning about each arc's center
as it follows the arc. Round the concave arc it lies in the notch, between the arc and its center,
where its radius of turning, 8 − s, is never less than 2.

**As a set:**

$$\bigl\{\, p : z \ge 0,\ (x, y) \notin Q,\ d\bigl((x, y), Q\bigr)^2 + z^2 \le 36 \,\bigr\}$$

where $Q$ is the plane region the path bounds.
