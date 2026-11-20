+++
title = "A tee of two thin-walled tubes, the stub bored through the main tube's wall"
tags = ["boolean", "cut", "fuse", "cylinder", "tube"]
origin = "OCCT tests/bugs/modalg_7/bug25968"
settled = false
+++

A pipe tee. The main tube lies along the x axis from x = −20 to x = 20: outer radius 5, bore
radius 4. The stub is a tube along the y axis from y = 0 to y = 10, at x = 0 and z = 0: outer
radius 3, bore radius 2. The stub starts on the main tube's axis, inside its bore, and runs out
through its wall.

The solid is the two outer cylinders joined, less the two bores joined: the stub's bore opens into
the main tube's bore, through the main tube's wall, and the stub's wall is fused to the main
tube's wall around that opening. It is one solid with its two bores one cavity, open at the main
tube's two ends and the stub's end.

The curve where a cylinder of radius 3 meets one of radius 5 at right angles, and the one where
the bore of radius 2 meets the bore of radius 4, have no closed form for the volume and the area
(they involve elliptic integrals), so the case is unsettled until two kernels agree.

**As a set:**

$$\bigl(\{\,y^2 + z^2 \le 25,\ |x| \le 20\,\} \cup \{\,x^2 + z^2 \le 9,\ 0 \le y \le 10\,\}\bigr) \setminus \bigl(\{\,y^2 + z^2 \le 16,\ |x| \le 20\,\} \cup \{\,x^2 + z^2 \le 4,\ 0 \le y \le 10\,\}\bigr)$$
