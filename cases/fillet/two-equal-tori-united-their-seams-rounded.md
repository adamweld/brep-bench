+++
title = "Two equal tori united, the curves where they cross rounded"
tags = ["fillet", "fuse", "torus", "concave", "face-blend"]
origin = "OCCT tests/bugs/modalg_7/bug28119_1"
settled = false
+++

Two tori, each with major radius 10 and minor radius 3: the first about the z axis through the
origin, the second the same torus moved 10 along x, so that its centre circle passes through the
first one's centre. Both lie flat, in the xy plane. United, they are one solid.

Where the two surfaces cross they leave creases: the curves of the first torus's surface that are
inside the second, and so on, which all run in the one solid's boundary. Round every one of those
crossing curves to radius 1: the blend a ball of radius 1 sweeps as it rolls touching both
surfaces, adding material where the crease is concave and taking it away where it is convex. The
two tori have exactly the same minor radius, which makes their crossing curves meet one another
in a symmetric way, in the plane x = 5, where a kernel's intersection easily goes wrong.

There is no closed form, so the case is unsettled until two kernels agree.

**As a set:**

$$T_1 \cup T_2,\ T_1 = \{\,(\rho - 10)^2 + z^2 \le 9\,\},\ T_2 = T_1 \text{ moved by } (10, 0, 0),$$

with the creases of its boundary rounded to radius 1.
