+++
title = "A ruled elliptic loft united with an ellipsoid and an elliptic cylinder"
tags = ["loft", "ruled", "fuse", "ellipsoid", "ellipse", "freeform"]
origin = "OCCT tests/bugs/modalg_7/bug26077_2"
settled = false
+++

Three bodies, overlapping.

**The loft** is ruled, straight between two ellipses, closed by flat ellipse faces: the ellipse in
the plane z = 0 centred on the z axis with semi-axis 4 along x and 2 along y, and the one in the
plane z = 8 centred on the z axis with semi-axis 2 along x and 4 along y. Each ellipse is
parametrized counterclockwise seen from +z, starting at the end of its longer axis, the +x end of
the lower one and the +y end of the upper one; the points of equal parameter fraction are joined
by straight lines.

**The ellipsoid** is $x'^2/25 + y'^2/9 + z'^2/16 \le 1$ where (x', y', z') = (x − 2, y, z − 4):
semi-axes 5, 3 and 4 about the point (2, 0, 4), axes along x, y and z.

**The elliptic cylinder** is the set $(x + 3)^2/9 + y^2/4 \le 1$, from z = 2 to z = 6.

United: the loft with the ellipsoid, and the cylinder with that. The result is one solid. The
ellipsoid is not a ball: the faces of the loft and of the cylinder cross it along curves no closed
form describes, so the case is unsettled until two kernels agree.
