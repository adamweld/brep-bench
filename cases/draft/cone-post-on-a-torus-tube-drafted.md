+++
title = "A widening cone post on a torus's tube, its side drafted the other way"
tags = ["draft", "cone", "torus", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/F9, OCCT tests/draft/angle/K7"
+++

The torus: the solid swept by the disk of radius 5 centered on (15, 0, 0) in the plane y = 0,
turned a full turn about the z axis. The middle circle of its tube lies in the plane z = 0, and it
reaches up to z = 5.

A post stands on its tube: the cone about the vertical line through (15, 0), from z = 0 to
z = 16, its radius about that line 4 at z = 0 and growing as it rises, 4 + z·tan 2°. Its floor
lies inside the tube. It is united with the torus; its side shows only above the tube's surface.

Draft the post's side by 5°, with the pull direction +z and the neutral plane z = 0, the plane of
the torus's middle: the side turns about the circle of radius 4 about (15, 0) where it crosses the
neutral plane, until it leans 5° from the z direction, inward as it rises. It becomes the cone
through that circle with radius 4 − z·tan 5° about the vertical line, narrowing to about 2.60 at
z = 16. The torus and the post's top are not drafted; the top is cut back to meet the cone.
It has no closed form that has been checked.

**As a set:**

$$\{\, (\rho - 15)^2 + z^2 \le 25 \,\} \cup \{\, \sqrt{(x - 15)^2 + y^2} \le 4 - z\tan 5^\circ,\ 0 \le z \le 16 \,\}$$
