+++
title = "A turned post on a torus, its outer side drafted"
tags = ["draft", "cone", "torus", "revolve", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/G2"
+++

The torus: the solid swept by the disk of radius 5 centered on (15, 0, 0) in the plane y = 0,
turned a full turn about the z axis. The middle circle of its tube lies in the plane z = 0, and it
reaches up to z = 5.

A post stands on its tube, turned about the same z axis: the region 13 ≤ ρ ≤ 17, 0 ≤ z ≤ 12 of the
half plane y = 0, x > 0, turned 120° about the z axis from +x toward +y. It is a curved block
riding on the tube's crest, with an outer face on the cylinder ρ = 17, an inner face on ρ = 13, a
flat top at z = 12 and two flat ends in the planes through the axis at 0° and 120°. Its floor lies
inside the tube. It is united with the torus.

Draft the post's outer face by 4°, with the pull direction +z and the neutral plane z = 0, the
plane of the torus's middle: the face turns about the arc of radius 17 where it crosses the
neutral plane, until it leans 4° from the z direction, inward as it rises, into the cone
ρ = 17 − z·tan 4°. The inner face, the two ends, the top and the torus are not drafted; the top is
cut back to meet the cone. It has no closed form that has been checked.

**As a set:**

$$\{\, (\rho - 15)^2 + z^2 \le 25 \,\} \cup \{\, 13 \le \rho \le 17 - z\tan 4^\circ,\ 0 \le z \le 12,\ 0^\circ \le \theta \le 120^\circ \,\}$$
