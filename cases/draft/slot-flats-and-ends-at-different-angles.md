+++
title = "A slot-shaped block, its flats and round ends drafted at different angles"
tags = ["draft", "cone", "tangent", "analytic"]
origin = "OCCT tests/bugs/modalg_2/bug421"

[volume]
value = 2483.4872677242656
exact = "∫₀¹⁰ A(z) dz, A = 4Lh + 2(h√(ρ² − h²) + ρ² asin(h/ρ)), h = 5 − z tan 6°, ρ = 5 − z tan 3°, L = 10: the section is the band |y| ≤ h across the slot of radius ρ"
tex = '\int_0^{10} A(z)\,dz,\quad A = 4Lh + 2\left(h\sqrt{\rho^2 - h^2} + \rho^2\arcsin\tfrac{h}{\rho}\right),\quad h = 5 - z\tan 6^\circ,\ \rho = 5 - z\tan 3^\circ,\ L = 10'

[area]
value = 1195.6257316565657
exact = "A(0) + A(10) + (2/cos 6°)∫₀¹⁰ (2L + 2√(ρ² − h²)) dz + (2/cos 3°)∫₀¹⁰ 2ρ asin(h/ρ) dz, A(0) = 200 + 25π: the bottom, the top, the two flats and the two cones"
tex = 'A(0) + A(10) + \frac{2}{\cos 6^\circ}\int_0^{10}\left(2L + 2\sqrt{\rho^2 - h^2}\right)dz + \frac{2}{\cos 3^\circ}\int_0^{10} 2\rho\arcsin\tfrac{h}{\rho}\,dz'
+++

A slot-shaped block: the outline in the plane z = 0 made of the segments from (−10, −5) to
(10, −5) and from (10, 5) to (−10, 5) and the half circles of radius 5 about (10, 0) and (−10, 0)
that join them, extruded from z = 0 to z = 10. Its side is four faces: two flats, y = 5 and
y = −5, and two half cylinders, tangent to the flats along the lines x = ±10.

Draft its sides in one operation, with the pull direction +z and the neutral plane z = 0, each
leaning inward as it rises: the two flats by 6° and the two round ends by 3°. Each face turns
about the curve where it meets the base. A flat becomes the plane through its bottom edge leaning
6° from the z direction, so at height z it is the line |y| = h = 5 − z tan 6°. A round end becomes
the cone through its bottom half circle leaning 3° from the z direction, so at height z it is the
circle of radius ρ = 5 − z tan 3° about (±10, 0). The top, z = 10, is not drafted; it shrinks to
meet them.

The flats lean more than the ends, so above the base each flat cuts into the cone beside it
rather than meeting it tangentially: the flat runs on past x = ±10 until it meets the cone, along
a curve where the two cross, and the cone is trimmed there. The section at height z is the band
|y| ≤ h across the slot whose round ends have radius ρ: the rectangle |x| ≤ 10, |y| ≤ h and the
parts of the two disks of radius ρ within the band. Each flat and each cone beside it are tangent
only at the base, at the points (±10, ±5, 0), where h = ρ = 5.

**As a set:**

$$\{\, |y| \le 5 - z\tan 6^\circ,\ 0 \le z \le 10 \,\} \cap \Bigl(\{\, |x| \le 10 \,\} \cup \{\, (|x| - 10)^2 + y^2 \le (5 - z\tan 3^\circ)^2 \,\}\Bigr)$$
