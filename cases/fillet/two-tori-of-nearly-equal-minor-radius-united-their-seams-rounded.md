+++
title = "Two tori of nearly equal minor radius united, the curves where they cross rounded"
tags = ["fillet", "fuse", "torus", "concave", "face-blend"]
origin = "OCCT tests/bugs/modalg_7/bug28119_2, OCCT tests/bugs/modalg_7/bug28119_3"
settled = false
+++

Two tori in the xy plane, both with major radius 10: the first about the z axis through the
origin, with minor radius 3; the second moved 10 along x, with minor radius 2.9, a little smaller.
United, they are one solid, whose boundary has creases where the two surfaces cross, and where the
smaller one's tube comes out of the larger one's.

Round every crossing curve to radius 1: the blend a ball of radius 1 sweeps as it rolls touching
both surfaces, adding material where the crease is concave and taking it away where it is convex.
Because the minor radii differ, the two crossing curves of the equal-radius pair (see
`fillet/two-equal-tori-united-their-seams-rounded`) no longer meet in the plane x = 5 but pull apart.

There is no closed form, so the case is unsettled until two kernels agree.

**As a set:**

$$T_1 \cup T_2,\ T_1 = \{\,(\rho - 10)^2 + z^2 \le 9\,\},\ T_2 = \{\,(\rho' - 10)^2 + z^2 \le 2.9^2\,\},\ \rho' = \sqrt{(x - 10)^2 + y^2},$$

with the creases of its boundary rounded to radius 1.
