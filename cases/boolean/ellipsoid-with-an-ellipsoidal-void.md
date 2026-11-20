+++
title = "An ellipsoid with a concentric ellipsoid cut out of it"
tags = ["boolean", "cut", "ellipsoid", "void", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestPrimitive.py::testPrimitiveEllipsoid"

[volume]
value = 175.92918860102842
exact = "4π(6·4·3 − 5·3·2)/3 = 4π(72 − 30)/3 = 56π"
tex = '\tfrac{4\pi}{3}\left(6\cdot 4\cdot 3 - 5\cdot 3\cdot 2\right) = 56\pi'

[area]
value = 366.02339174087707
exact = "S(6, 4, 3) + S(5, 3, 2), the ellipsoid's area S(a, b, c) = 2πc² + (2πab/sin φ)(E(φ, k) sin²φ + F(φ, k) cos²φ), a > b > c, cos φ = c/a, k² = a²(b² − c²)/(b²(a² − c²)); the two surfaces, 231.248215… and 134.775176…"
+++

An ellipsoid centered on the origin with semi-axes 6 along x, 4 along y and 3 along z, with a smaller
ellipsoid on the same center, its semi-axes 5 along x, 3 along y and 2 along z, cut out of it. The
smaller lies wholly inside the larger and touches it nowhere, so what is left is a closed hollow
ellipsoid: one solid bounded by two shells, the outer facing out and the cavity's facing in.

**As a set:**

$$\{\,\tfrac{x^2}{36} + \tfrac{y^2}{16} + \tfrac{z^2}{9} \le 1\,\} \setminus \{\,\tfrac{x^2}{25} + \tfrac{y^2}{9} + \tfrac{z^2}{4} < 1\,\}$$
