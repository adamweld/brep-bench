+++
title = "A quarter of a domed button, one flat moved out"
tags = ["edit", "move-face", "revolved", "cone", "ellipsoid", "concave-edge", "unsettled"]
origin = "OCCT tests/offset/faces_type_i/E7"
settled = false
+++

The button: the solid of revolution about the z axis of the region of the half-plane ρ ≥ 0 (ρ the
distance from the axis) bounded by the axis from z = 0 to z = 5, the base, ρ from 0 to 6 at z = 0,
the straight line from (ρ, z) = (6, 0) to (4, 2), the flare, and the quarter of the ellipse (ρ/4)² +
((z − 2)/3)² = 1 from (4, 2) to (0, 5), the dome. The flare meets the dome at a concave edge, the
circle ρ = 4 at z = 2, and the dome rises from it vertically. Take a quarter of it: the part whose
angle about the z axis, φ, measured from +x toward +y, lies between 0° and 90°. Its faces are the
base, a quarter of a disk; the flare, a quarter of a cone; the dome, a quarter of an ellipsoid of
revolution; and two flat faces through the axis, each the half-section above, in the half-planes y =
0, x ≥ 0 (φ = 0°) and x = 0, y ≥ 0 (φ = 90°).

Then edit the solid directly: move the flat at φ = 0° by 1 outward along its normal, −y, to the
plane y = −1. The faces beside it follow it: the base, the flare and the dome run on, as the same
surfaces, into the part before φ = 0° as far as that plane, past the axis; and the other flat, in x
= 0, runs on past the axis from y ≥ 0 to y ≥ −1, as the same half-section reflected.

So the result is the whole button's part with x ≥ 0 and y ≥ −1.

**As a set:**

$$\{\, (\rho,\, z) \in \text{section} \,\} \cap \{\, x \ge 0,\ y \ge -1 \,\}$$
