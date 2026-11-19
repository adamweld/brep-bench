+++
title = "A block grooved by its top face turned a full turn about a bottom edge"
tags = ["feature", "revolve", "groove", "tangent", "cylinder", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestTopologicalNamingProblem.py testPartDesignElementMapGroove"

[volume]
value = 6741.006652759706
exact = "L·S, S = ∫₀¹² √(20² − u²) du = 96 + 200 arcsin(3/5) the section, L = 30"
tex = 'L\,S,\quad S = \int_0^{12}\sqrt{20^2 - u^2}\,du = 96 + 200\arcsin\tfrac{3}{5},\quad L = 30'

[area]
value = 2275.5011087932844
exact = "2S + L(12 + 20 + 16 + 20 arcsin(3/5)): the two ends, the bottom, the side x = 12, the side x = 0 up to z = 16 and the arc's cylinder"
tex = '2S + L\left(12 + 20 + 16 + 20\arcsin\tfrac{3}{5}\right)'
+++

A block from the origin to (12, 30, 20). Its top face, the rectangle 0 ≤ x ≤ 12, 0 ≤ y ≤ 30 at
z = 20, is turned a whole turn about the line of the block's bottom edge at x = 12, z = 0 (the line
parallel to the y axis through (12, 0, 0)), and everything it passes through is removed from the
block: a groove.

The top face's points lie between 20 from that line (its edge x = 12, straight above it) and
√(12² + 20²) = √544 (its edge x = 0). So the groove is every point between those two distances
from the line, and it takes away all of the block that is more than 20 from the line.

What is left is the part of the block within 20 of the line, a prism 30 long. Its section, in the
plane of x and z, is the rectangle 12 wide and 20 tall less its corner beyond the arc of radius 20
about (x, z) = (12, 0). The arc meets the block's top at the edge x = 12, tangent to it there, and
the side x = 0 at z = 16. So the solid's faces are its two ends, the bottom, the whole side x = 12,
the side x = 0 up to z = 16, and the arc's cylinder.

**As a set:**

$$K \setminus \{\,20^2 \le (x - 12)^2 + z^2 \le 544\,\}, \quad K = [0, 12] \times [0, 30] \times [0, 20]$$
