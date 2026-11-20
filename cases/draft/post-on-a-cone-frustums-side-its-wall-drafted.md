+++
title = "A round post against a cone frustum's side, its wall drafted"
tags = ["draft", "cone", "boolean"]
settled = false
origin = "OCCT tests/draft/angle/E5"
+++

A frustum of a cone about the z axis, from z = 0 to z = 20, its section at height z the disk of
radius 10 − z/5, and against its side a post: a cylinder of radius 4 about the vertical line through
(11, 0), from z = 5 to z = 15, united with it. The post overlaps the frustum, whose wall cuts through
it, so the post stands out of the wall along a curve.

Draft the post's wall by 5°, with the pull direction +z and the neutral plane z = 5, the post's
floor: the wall becomes a cone through the circle of radius 4 about (11, 0) at z = 5, leaning 5°
from the z direction, inward as it rises, its radius 4 − (z − 5)·tan 5° at height z. The post's floor
and top and the frustum are not drafted; the top shrinks to meet the cone, and the cone ends on the
frustum's wall in a curve. It has no closed form that has been checked.

**As a set:**

$$\{\,\rho \le 10 - z/5,\ 0 \le z \le 20\,\} \cup \{\,(x-11)^2 + y^2 \le (4 - (z-5)\tan 5^\circ)^2,\ 5 \le z \le 15\,\}$$
