+++
title = "A hollow frustum lofted between two rings, with a tube padded on its top"
tags = ["loft", "ruled", "extrude", "cone", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestLoft.py testPadOnLoftBetweenCones"

[volume]
value = 12867.963509103793
exact = "πh/3·(R₁² + R₁R₂ + R₂²) for each frustum, h = 24, and the tube: 8π(20² + 20·12 + 12²) − 8π(14² + 14·8 + 8²) + π(12² − 8²)·10 = 4096π"
tex = '8\pi\left(20^2 + 20\cdot 12 + 12^2\right) - 8\pi\left(14^2 + 14\cdot 8 + 8^2\right) + 10\pi\left(12^2 - 8^2\right) = 4096\pi'

[area]
value = 6401.915592430026
exact = "π(20² − 14²) + π(12² − 8²) + π(20 + 12)√(24² + 8²) + π(14 + 8)√(24² + 6²) + 2π(12 + 8)·10 = 684π + 256√10π + 132√17π"
tex = '\pi\left(20^2 - 14^2\right) + \pi\left(12^2 - 8^2\right) + 32\pi\sqrt{24^2 + 8^2} + 22\pi\sqrt{24^2 + 6^2} + 400\pi = 684\pi + 256\sqrt{10}\,\pi + 132\sqrt{17}\,\pi'
+++

A loft between two rings on the z axis, and then a pad on its top.

The loft: from the ring between radii 14 and 20 in the plane z = 0 to the ring between radii 8 and
12 in the plane z = 24, ruled. Each circle is joined by straight lines to the matching circle of the
other ring, outer to outer and inner to inner. So the loft is a hollow frustum: the frustum of a
cone from radius 20 at z = 0 to 12 at z = 24, less the frustum from radius 14 to 8.

The pad: the loft's flat top, the ring between radii 8 and 12 at z = 24, extruded up 10 to z = 34
and joined to it. It is a tube whose walls carry on from the frustum's rims, each turning there
from a cone's slope to upright.

**As a set:**

$$\{\,14 - z/4 \le \rho \le 20 - z/3,\ 0 \le z \le 24\,\} \cup \{\,8 \le \rho \le 12,\ 24 \le z \le 34\,\}$$
