+++
title = "A bar laid across the mouth of a tube"
tags = ["feature", "extrude", "start-on-face", "union", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestPad.py::testStartOffsetAndReference; src/Mod/PartDesign/PartDesignTests/TestPad.py::testStartReferenceInternalSketchFace"

[volume]
value = 6626.548245743669
exact = "tube π(14² − 6²)·10 = 1600π, bar 40·8·5 = 1600; they touch only on the strip of the bar's underside over the tube's top"
tex = '1600\pi + 1600'

[area]
value = 3116.7538085954866
exact = "tube 2π(14² − 6²) + 2π·14·10 + 2π·6·10 = 720π, bar 2·40·8 + 2·40·5 + 2·8·5 = 1120, less the contact, which each loses: O = S(14) − S(6) with S(r) = 8√(r² − 16) + 2r² arcsin(4/r) the area of the strip |y| ≤ 4 of the disc of radius r: 720π + 1120 − 64√5 − 784 arcsin(2/7) + 144 arcsin(2/3)"
+++

A bar laid across the mouth of a tube, starting on the tube's top face.

The tube is the cylinder of radius 14 about the z axis from z = 0 to z = 10, with a bore of radius 6
through it: 6 ≤ ρ ≤ 14, 0 ≤ z ≤ 10. The bar is the rectangle |x| ≤ 20, |y| ≤ 4 in the plane z = 10,
which is the tube's top face, extruded up 5 to z = 15 and joined to the tube. It lies across the middle of
the tube's top and overhangs it by 6 at each end, so part of its underside lies over the tube's top ring and
part over the bore, which it covers only in a strip |y| ≤ 4; the bore stays open on both sides of the strip
and at the bottom.

**As a set:**

$$\{\,6 \le \rho \le 14,\ 0 \le z \le 10\,\} \cup [-20, 20] \times [-4, 4] \times [10, 15]$$
