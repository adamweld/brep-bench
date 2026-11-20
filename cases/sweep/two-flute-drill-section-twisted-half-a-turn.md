+++
title = "A two-fluted drill section twisted half a turn"
tags = ["sweep", "twisted", "helix", "arcs", "analytic"]
origin = "OCCT bugs/modalg_5/bug24758_2"

[volume]
value = 255.0894271880949
exact = "12·(9π − 2L), L the lens where each flute disk meets the radius-3 disk (R = 3, r = 1.9, d = 3.4): L = r²·acos((d² + r² − R²)/2dr) + R²·acos((d² + R² − r²)/2dR) − ½√((−d + r + R)(d + r − R)(d − r + R)(d + r + R)); turning does not change a section's area"
tex = '12\left(9\pi - 2L\right),\quad L = r^2\arccos\tfrac{d^2 + r^2 - R^2}{2dr} + R^2\arccos\tfrac{d^2 + R^2 - r^2}{2dR} - \tfrac{1}{2}\sqrt{(-d + r + R)(d + r - R)(d - r + R)(d + r + R)}'

[area]
value = 292.4220672768434
exact = "2(9π − 2L) (the ends) + 2·3·(π − 2a)·12 (the two lobes' arcs, a = atan2(y₀, x₀) the half-angle each flute takes from the radius-3 circle; a turned cylinder's wall keeps its area) + 2·12·r·2E(b | −k²d²) (the two flutes), where b = atan2(y₀, d − x₀) is the half-angle of a flute arc about its center, k = 15° per unit = π/12, and E(φ | m) = ∫₀^φ √(1 − m sin²θ) dθ the incomplete elliptic integral of the second kind: a wall point u along its arc from the foot of the axis's perpendicular has area element √(1 + k²g²) per unit of arc and of height, g = (center)·(tangent) = −d sin θ on a flute arc"
tex = '2(9\pi - 2L) + 2\cdot 3(\pi - 2a)\cdot 12 + 2\cdot 12\cdot r\cdot 2E\left(b \mid -k^2d^2\right),\quad k = \tfrac{\pi}{12}'
+++

A twist-drill-like bar: a round section with two circular flutes cut out of it, turned steadily as it rises along a straight axis, half a turn in all.

The solid runs from z = 0 to z = 12. Its section at z = 0 is the disk of radius 3 about the z axis less two round flutes: the disks of radius 1.9 centered on (3.4, 0) and (−3.4, 0). What is left is one piece, a web 3 wide across its middle (|x| ≤ 1.5 at y = 0) with a lobe at each end. Its boundary is two arcs of the radius-3 circle, one at the top and one at the bottom (each about 112.4° round), and two concave arcs of the flute circles (each about 122.9° round), meeting in four corners at (±2.4926, ±1.6692), where the radius-3 circle meets a flute circle. The section at every height z is this one turned about the z axis, right-handed (from +x toward +y), by 15° for every unit of height: at z = 6 a quarter turn, and at z = 12 a half turn, which brings the section back onto itself.

The flutes' edges run along helices: a corner of the section at distance ρ from the axis runs on a helix of radius ρ that rises 24 in a turn. The solid is swept from the section along the straight axis, turned about it by that guide helix. Its two ends are flat, the section at z = 0 and its turned copy at z = 12; its other faces are two helicoidal strips of the radius-3 cylinder and the two flute surfaces, swept by the section's arcs.

**As a set:**

$$\{\,0 \le z \le 12,\ u^2 + v^2 \le 9,\ (u - 3.4)^2 + v^2 \ge 1.9^2,\ (u + 3.4)^2 + v^2 \ge 1.9^2\,\},\quad (u, v) = (x\cos\alpha + y\sin\alpha,\ -x\sin\alpha + y\cos\alpha),\ \alpha = 15^\circ z$$
