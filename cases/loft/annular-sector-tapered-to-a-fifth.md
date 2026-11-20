+++
title = "An annular sector lofted along a straight spine to a fifth of its size"
tags = ["loft", "ruled", "cone", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug23162_1"

[volume]
value = 12.985249634837812
exact = "(φ/2)(R² − r²)·(H/3)(1 − k³), φ = π/3, R = 4, r = 2, H = 25/4 the height of the common apex, k = 1/5: the section at height z is the sector scaled by 1 − z/H, so V = ∫₀⁵ (φ/2)(R² − r²)(1 − z/H)² dz = 62π/15"
tex = '\int_0^5 \tfrac{\varphi}{2}\,(R^2 - r^2)\left(1 - \tfrac zH\right)^2 dz = \tfrac{62\pi}{15},\quad \varphi = \tfrac\pi3,\ R = 4,\ r = 2,\ H = \tfrac{25}{4}'

[area]
value = 40.051179723897706
exact = "the two sectors 2π and 2π/25; the two flat sides, each a trapezoid with parallel sides 2 and 2/5 a height 5 apart, 6 each; the two conical walls (φ/2)(24/25)·(R√(R² + H²) + r√(r² + H²)) = (4π/25)(√881 + √689/2). In all 52π/25 + 12 + (4π/25)(√881 + √689/2)"
tex = '\tfrac{52\pi}{25} + 12 + \tfrac{4\pi}{25}\left(\sqrt{881} + \tfrac{\sqrt{689}}{2}\right)'
+++

A ruled loft along a straight spine between two annular sectors, the second a fifth the size of the
first. The first section lies in the plane z = 0: the part of the annulus 2 ≤ ρ ≤ 4 about the z axis
whose angle φ, measured from +x toward +y, runs from 0° to 60°. Its boundary is an arc of radius 2
from (2, 0, 0) to (1, √3, 0), a straight edge outward to (2, 2√3, 0), an arc of radius 4 back to
(4, 0, 0), and a straight edge inward to the start. The second section lies in the plane z = 5 and is
the first scaled by 1/5 about the z axis: radii 0.4 and 0.8, the same 0° to 60°.

Loft between them, each edge of the first joined to the matching edge of the second by straight
lines (a ruled loft, not smoothed), and close the ends with the two sectors. The solid has six faces:
the two sectors, two pieces of cone (the arcs' walls, with their common apex on the z axis at
z = 6.25), and the two flat sides in the half-planes φ = 0° and φ = 60°, each a trapezoid.

**As a set:**

$$\{\,2\bigl(1 - \tfrac{z}{6.25}\bigr) \le \rho \le 4\bigl(1 - \tfrac{z}{6.25}\bigr),\ 0 \le \varphi \le 60^\circ,\ 0 \le z \le 5\,\}$$
