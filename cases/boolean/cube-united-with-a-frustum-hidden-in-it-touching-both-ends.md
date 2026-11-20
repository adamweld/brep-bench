+++
title = "A cube united with a cone frustum as tall as it, inside it"
tags = ["boolean", "union", "analytic", "cone", "coincident", "face-contact"]
origin = "OCCT tests/boolean/bopfuse_simple/ZF7"

[volume]
value = 216.0
exact = "the cube is unchanged: 6³"
tex = '6^3 = 216'

[area]
value = 216.0
exact = "6·6²"
tex = '6\cdot 6^2 = 216'
+++

A cube of edge 6 from (−3, −3, 0) to (3, 3, 6), and a cone frustum inside it on the z axis: base radius 2 at z = 0, top radius 1 at z = 6, so its base disc lies in the cube's bottom face and its top disc in the cube's top face. Make their union.

The frustum adds nothing: its wall is inside the cube and touches nothing, and its two end discs lie in the cube's end faces. What is left is the cube, whole. (A frustum that lay wholly inside the cube without touching its faces would likewise leave it unchanged.)

**As a set:**

$$[-3,3]\times[-3,3]\times[0,6] \cup \{\rho \le 2 - z/6,\ 0 \le z \le 6\}$$
