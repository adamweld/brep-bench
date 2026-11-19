+++
title = "A cone frustum through a block, both junctions chamfered by their throat"
tags = ["chamfer", "throat", "concave", "cone", "analytic"]
origin = "OCCT tests/bugs/modalg_7/bug28828_17"

[volume]
value = 4236.520607955972
exact = "4000 + 143π/2 (the block and the frustum's two ends outside it) + Σ 2πρ̄·d² sin θ/2 over the two junctions (Pappus): the legs d = t/cos(θ/2), t = 1/2, θ the angle of the groove, cos θ = ∓1/√17 (top, bottom), sin θ = 4/√17, ρ̄ the mean of the triangle's corners' ρ: (5/2, 5/2 + d, 5/2 − d/√17) at the top and (5, 5 + d, 5 + d/√17) at the bottom"
tex = '4000 + \tfrac{143\pi}{2} + \sum_{j} 2\pi\bar\rho_j\,\tfrac{1}{2}d_j^2\sin\theta_j,\quad d_j = \frac{t}{\cos(\theta_j/2)},\ t = \tfrac12,\ \cos\theta_{\mathrm{top}} = -\tfrac{1}{\sqrt{17}},\ \cos\theta_{\mathrm{bot}} = \tfrac{1}{\sqrt{17}}'

[area]
value = 1704.2767090146474
exact = "1600 − 25π − 25π/4 + 21π√17/4 + 4π√17 + 121π/4 + 9π/4 (the block, the frustum outside it and its ends) + Σ over the junctions of π(ρ_P + ρ_C)·2d sin(θ/2) − π((ρ_J + d)² − ρ_J²) − π(ρ_J + ρ_C)d: the chamfer's cone added, the ring of the block's face and the band of the frustum's side it covers taken away"
tex = '1600 - \tfrac{125\pi}{4} + \tfrac{37\pi\sqrt{17}}{4} + \tfrac{130\pi}{4} + \sum_j\Bigl(\pi(\rho_P + \rho_C)\,2d_j\sin\tfrac{\theta_j}{2} - \pi\bigl((\rho_J + d_j)^2 - \rho_J^2\bigr) - \pi(\rho_J + \rho_C)\,d_j\Bigr)'
+++

A block from (−10, −10, 0) to (10, 10, 10), joined to a cone frustum on the z axis that runs
right through it: radius 5.5 at z = −2 narrowing to 1.5 at z = 14, so its radius is ρ = 5 − z/4,
5 where it meets the block's bottom face and 2.5 where it meets the top. Both junctions are
concave circular edges: the circle of radius 2.5 at z = 10 and the circle of radius 5 at z = 0.

Chamfer both edges by a throat of 0.5, as a weld is sized. A concave chamfer adds material: it
fills the groove between the frustum and the block's face. In the plane through the z axis (the
plane square to the edge), the chamfer's section is an isosceles triangle: its two legs, along
the block's face and along the frustum's side from the edge, are equal, and its third side, the
chamfer's face, lies 0.5 from the edge. The legs are d = 0.5/cos(θ/2), θ being the angle of the
groove between the face and the side, which is not the same at the two junctions:

- at the top, the side leans in over the face: θ = 90° + atan(1/4) ≈ 104.04°, and d ≈ 0.81246,
  so the chamfer meets the top face on the circle of radius 2.5 + d and the side on the circle at
  z = 10 + 4d/√17;
- at the bottom, the side leans out under the face: θ = 90° − atan(1/4) ≈ 75.96°, and
  d ≈ 0.63435, so the chamfer meets the bottom face on the circle of radius 5 + d and the side on
  the circle at z = −4d/√17.

Each chamfer is the cone between its two circles, and the solid is the block and the frustum
joined, with the two rings between them and the cones filled in.

**As a set:**

$$K \cup \{\, z \ge 10,\ \rho \ge 5 - \tfrac{z}{4},\ (\rho - \tfrac52,\ z - 10)\cdot n_1 \le \tfrac12 \,\} \cup \{\, z \le 0,\ \rho \ge 5 - \tfrac{z}{4},\ (\rho - 5,\ z)\cdot n_2 \le \tfrac12 \,\}$$

where K is the block and the frustum joined, and n₁ and n₂ are the unit vectors in (ρ, z) halfway
between the legs: n₁ along (1, 0) + (−1, 4)/√17 and n₂ along (1, 0) + (1, −4)/√17.
