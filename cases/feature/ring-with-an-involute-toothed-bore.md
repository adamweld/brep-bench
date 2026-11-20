+++
title = "A ring whose bore is a twelve-tooth gear outline"
tags = ["feature", "pocket", "gear", "involute", "through-all", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestInvoluteGear.py::testUsagePocketInternalGearProfile"

[volume]
value = 6820.027820061887
exact = "8(400π − A), A the gear outline's area, 81π − 1056.9 + 1875 arccos(4/5) (derived in the spur gear of twelve involute teeth)"
tex = '8\left(400\pi - A\right)'

[area]
value = 3773.5086073850184
exact = "2(400π − A) + 2π·20·8 + 8P, P the gear outline's perimeter, 300 arccos(4/5) + 18π − 116.7"
tex = '2\left(400\pi - A\right) + 320\pi + 8P'
+++

A round blank with a toothed bore: a cylinder with a gear-shaped hole cut right through it.

The blank is the cylinder of radius 20 about the z axis, from z = 0 to z = 8. The hole is the twelve-tooth
gear outline below, through both ends. (The hole has the *shape of the gear*, so the material between its
lobes points inward as the gear's gaps do.)

The outline is twelve identical teeth about the z axis, the n-th tooth centred on the direction at angle
30°·n from +x (n = 0 to 11). It is described by how much of each circle about the origin it holds. Let
ρ be the distance from the z axis and φ the angle from +x, and let w(ρ) be a half-width in radians:

- for ρ ≤ 9, the whole circle;
- for 9 < ρ ≤ 10, w = 0.2;
- for 10 < ρ ≤ 12.5, w = 0.2 − (tan α − α), where α = arccos(10/ρ);
- beyond 12.5, nothing.

A point at distance ρ and angle φ is inside the outline if, for some tooth, its angle differs from that
tooth's centre by at most w(ρ). So each tooth is bounded, going round it from its root, by: an arc of the
circle ρ = 9 along the gap before it; a straight radial segment from ρ = 9 to ρ = 10 at 0.2 rad from the
tooth's centre; an **involute** of the circle of radius 10, the curve a taut string traces as it is
unwound from that circle, leaving the circle along the radial segment and running out to ρ = 12.5; an arc
of the circle ρ = 12.5 across the tip, 2(0.2 − 0.75 + arccos 0.8) rad wide; the other involute, back to
ρ = 10, and the other radial segment down to ρ = 9. The gaps between teeth are arcs of ρ = 9, each
(π/6 − 0.4) rad wide. Each flank is tangent to its radial segment, and the tip and root arcs meet the
flanks at corners.

So the hole is the whole region this outline encloses, and what is left of the blank is the ring between
the outline and the circle of radius 20: its bore is the gear's teeth and gaps, and its flanks are twelve
pairs of involute surfaces.

**As a set:**

$$\{\,\rho \le 20,\ 0 \le z \le 8\,\} \setminus \{\,\min_n |\varphi - 30^\circ n| \le w(\rho)\,\}$$
