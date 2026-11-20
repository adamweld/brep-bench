+++
title = "A turned post on a dome, its outer side drafted"
tags = ["draft", "cone", "sphere", "revolve", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/F6"
+++

The dome: everything within 20 of the origin with z ≥ 0, a hemisphere whose flat face is the disk
z = 0 and whose pole is (0, 0, 20).

A post stands on it, turned about the z axis: the region 3 ≤ ρ ≤ 8, 0 ≤ z ≤ 28 of the half plane
y = 0, x > 0, turned 120° about the z axis from +x toward +y. It is a curved block, a third of a
tube, with an outer face on the cylinder ρ = 8, an inner face on ρ = 3, a flat top at z = 28 and
two flat ends in the planes through the axis at 0° and 120°. It is united with the dome; it shows
above the sphere only where it rises over it.

Draft the post's outer face by 4°, with the pull direction +z and the neutral plane z = 0, the
dome's flat base: the face turns about the arc of radius 8 where it crosses the neutral plane,
until it leans 4° from the z direction, inward as it rises, into the cone ρ = 8 − z·tan 4°. The
inner face, the two ends, the top and the dome are not drafted; the top is cut back to meet the
cone. It has no closed form that has been checked.

**As a set:**

$$\bigl(B_{20} \cap \{\, z \ge 0 \,\}\bigr) \cup \{\, 3 \le \rho \le 8 - z\tan 4^\circ,\ 0 \le z \le 28,\ 0^\circ \le \theta \le 120^\circ \,\}$$
