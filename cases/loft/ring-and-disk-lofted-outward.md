+++
title = "A ring and a disk lofted outward together"
tags = ["loft", "ruled", "concentric", "multi-body", "cone", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestLoft.py::testNestedIslandLoftIgnoresCreationOrder"

[volume]
value = 14524.63003509681
exact = "(10π/3)[(20² + 20·30 + 30²) − (12² + 12·18 + 18²) + (6² + 6·9 + 9²)] = 13870π/3"
tex = '\tfrac{10\pi}{3}\left[1900 - 684 + 171\right] = \tfrac{13870\pi}{3}'

[area]
value = 6793.909288957072
exact = "flats π(20² − 12²) + π6² + π(30² − 18²) + π9² = 949π, and three cone sides π(R₁ + R₂)√(10² + (R₂ − R₁)²): 50π√200 + 30π√136 + 15π√109 = 949π + 500√2π + 60√34π + 15√109π"
+++

A ring and a disk inside it, lofted outward together: a ring-shaped frustum shell and a frustum standing
in its hollow, two separate solids.

Two sets of concentric circles about the z axis. At z = 0 the radii are 20, 12 and 6, so the section is
the ring between 12 and 20 and the disk of radius 6 (the gap between 6 and 12 is empty). At z = 10 the
radii are 30, 18 and 9: the ring between 18 and 30 and the disk of radius 9. The loft joins the circles
in order, outer to outer (20 to 30), middle to middle (12 to 18), inner to inner (6 to 9), each pair by
straight lines, so the ring is a cone frustum's shell and the disk a cone frustum. The two do not touch (the gap between them runs from 6 to 12 at the foot and from 9
to 18 at the top).

**As a set:**

$$\{\,12 + 0.6z \le \rho \le 20 + z\,\} \cup \{\,\rho \le 6 + 0.3z\,\}, \quad 0 \le z \le 10$$
