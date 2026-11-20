+++
title = "A block rounded to its full thickness along one edge, with a post hanging tangent to two edges of its bottom"
tags = ["boolean", "union", "analytic", "fillet", "cylinder", "tangent"]
origin = "OCCT boolean/bfuse_simple/K6, OCCT boolean/bfuse_simple/K7, OCCT boolean/bfuse_simple/K8"

[volume]
value = 1088.4424506661589
exact = "12·(16·4 + π·4²/4) + π·3²·6"
tex = '12\,(16\cdot 4 + \tfrac{\pi}{4}4^2) + \pi\cdot 3^2\cdot 6'

[area]
value = 821.62830044410594
exact = "(192 + 240 + 48 + 2·(64 + 4π) + 12·2π) + 2π·3·6"
tex = '(192 + 240 + 48 + 2\,(64 + 4\pi) + 12\cdot 2\pi) + 2\pi\cdot 3\cdot 6'
+++

A block 20 long, 12 wide and 4 thick, from (0, 0, 0) to (20, 12, 4), with its top edge along x = 20 rounded by a fillet of radius 4, the block's thickness: in each plane y = const the profile is the rectangle 0 ≤ x ≤ 16, 0 ≤ z ≤ 4 with the quarter disc of radius 4 about the point (16, 0) added on its right, x ≥ 16, z ≥ 0. The round runs from the top face's edge at x = 16 down to the bottom face's edge at x = 20 and uses up the whole side face there; the block's other edges are sharp. And a cylinder of radius 3 hanging from the block's bottom face, from z = −6 to z = 0, its axis the vertical line through (17, 3). Unite them.

The cylinder's wall touches the bottom face's two straight edges: the edge x = 20 where the bottom meets the round, a sharp edge, and the edge y = 0. No volume overlaps. The union is the rounded block with the cylinder hanging from it, tangent to those two edge lines. If the cylinder you make has a seam, its place is yours; the solid does not depend on it.

**As a set:**

$$\{\,0 \le x \le 16,\ 0 \le y \le 12,\ 0 \le z \le 4\,\} \cup \{\,(x-16)^2 + z^2 \le 16,\ x \ge 16,\ z \ge 0,\ 0 \le y \le 12\,\} \cup \{\,(x-17)^2 + (y-3)^2 \le 9,\ -6 \le z \le 0\,\}$$
