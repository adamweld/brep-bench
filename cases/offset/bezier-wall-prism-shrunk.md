+++
title = "A prism with a Bézier wall shrunk by 0.5 with sharp edges"
tags = ["offset", "shrink", "nurbs", "bezier", "sharp-edge"]
origin = "OCCT tests/offset/shape_type_i/G3, OCCT tests/offset/shape_type_i/G4"
settled = false
+++

A prism over a profile with one curved side. The profile, in the xy plane, is bounded by the
straight segment from (0, 0) to (10, 0), the cubic Bézier curve from (10, 0) to (0, 8) with control
points (10, 0), (10, 5), (5, 8) and (0, 8) (B(t) = Σ Bᵢ(t) Pᵢ with the cubic Bernstein polynomials
B₀ = (1 − t)³, B₁ = 3t(1 − t)², B₂ = 3t²(1 − t), B₃ = t³, for t from 0 to 1, a single cubic B-spline
segment with its knots 0, 0, 0, 0, 1, 1, 1, 1), and the straight segment from (0, 8) back to (0, 0).
The prism is that profile extruded from z = 0 to z = 6. Its faces are the base and top, two flat sides
(x = 0 and y = 0), and the curved wall, the Bézier curve extruded; the curve bulges outward and
its radius of curvature is nowhere less than 6.

Shrink it by 0.5 with its edges kept sharp: move every face 0.5 inward, against its normal, the wall
along its own normals to the parallel surface 0.5 inside it, and extend the moved faces until they
meet, so that no round is added. The curved wall's parallel surface is no Bézier curve, so the
result has no closed form, and the case is unsettled until two kernels agree.
