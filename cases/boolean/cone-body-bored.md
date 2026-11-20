+++
title = "Two cones, fused, pocketed and bored"
tags = ["boolean", "union", "cut", "cone", "cylinder", "analytic"]
origin = "OCCT boolean/gdml_public/A5"

[volume]
value = 710.3271889460422
exact = "turning the section (Pappus): π/3·Σ(ρᵢ² + ρᵢρᵢ₊₁ + ρᵢ₊₁²)(zᵢ₊₁ − zᵢ) over the outline (0.5, 0), (6, 0), (3, 10), (2, 14), (1.75, 14), (0.5, 9)"
tex = '\tfrac{\pi}{3}\sum_i\left(\rho_i^2 + \rho_i\rho_{i+1} + \rho_{i+1}^2\right)\left(z_{i+1} - z_i\right)\ \text{over the outline}'

[area]
value = 539.9204634853867
exact = "π·Σ(ρᵢ + ρᵢ₊₁)·ℓᵢ over the same outline, ℓᵢ the length of each side: a ring at each end, the two cones' sides, the pocket's side and the bore's wall"
tex = '\pi\sum_i\left(\rho_i + \rho_{i+1}\right)\ell_i'
+++

A turned body on the z axis made of two cones fused, a conical pocket in its top and a bore through it.

The first cone is a frustum from radius 6 at z = 0 to radius 3 at z = 10, so that its side is ρ = 6 − 0.3z. On top of it stands a second, from radius 3 at z = 10 to radius 2 at z = 14, ρ = 3 − (z − 10)/4, ending in a flat top at z = 14. Then a pocket is cut from the top: the cone with its apex at (0, 0, 7), opening upward, whose radius is (z − 7)/4 (1.75 at the top), so that its apex lies inside the body and it leaves a ring 0.25 wide at the top. Last, a bore of radius 0.5 is cut along the axis, from the bottom up to the pocket, which is wider than the bore above z = 9: below that the bore is a cylinder through the solid, and above it the pocket is the inside.

If you cut the pocket and the bore with solids, make them run past the faces they open in.

The solid is what turns about the z axis from the section in the xz plane with the corners (0.5, 0), (6, 0), (3, 10), (2, 14), (1.75, 14) and (0.5, 9), joined in that order.

**As a set:**

$$\{\,\rho \ge 0.5,\ \rho \le r(z),\ 0 \le z \le 14\,\} \setminus \{\,\rho < \tfrac{z - 7}{4},\ z \ge 7\,\},\quad r(z) = 6 - 0.3z\ (z \le 10),\ \ 3 - \tfrac{z - 10}{4}\ (z \ge 10)$$
