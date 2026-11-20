+++
title = "A spur gear of twelve involute teeth"
tags = ["feature", "extrude", "gear", "involute", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestInvoluteGear.py::testUsagePadGearProfile"

[volume]
value = 3233.0686714254516
exact = "8·A, A = 81π + 12[0.2(12.5² − 9²) − 2I], I = ∫ from 10 to 12.5 of (tan α − α) ρ dρ = 100(T³/3 − αT²/2 + (T − α)/2) at T = 3/4, α = arccos(4/5) (substituting ρ = 10 sec α); so A = 81π − 1056.9 + 1875 arccos(4/5)"
tex = '8A,\quad A = 81\pi - 1056.9 + 1875\arccos\tfrac{4}{5}'

[area]
value = 1871.4591710771756
exact = "2A + 8P, the perimeter P = 12[tip arc 25(α − 0.55) + root arc 9(π/6 − 0.4) + two radial segments 2 + two involute flanks 10·T²] (an involute's length from its base circle is the base radius times t²/2, t = tan α) = 300 arccos(4/5) + 18π − 116.7"
tex = '2A + 8P,\quad P = 300\arccos\tfrac{4}{5} + 18\pi - 116.7'
+++

A spur gear, a flat outline of twelve involute teeth extruded into a prism.

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

The solid is that outline in the plane z = 0, extruded 8 up to z = 8. It has no bore.

**As a set:**

$$\{\,0 \le z \le 8,\ \min_n |\varphi - 30^\circ n| \le w(\rho)\,\}$$
