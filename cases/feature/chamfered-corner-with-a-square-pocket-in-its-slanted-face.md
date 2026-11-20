+++
title = "A cube with an edge chamfered and a square pocket cut square into the chamfer"
tags = ["feature", "pocket", "chamfer", "slanted", "analytic"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestTopologicalNamingProblem.py::testPartDesignTNPChamfer"

[volume]
value = 949.0
exact = "1000 − ½·3·3·10 − 2·2·1.5 = 949"
tex = '1000 - \tfrac12\cdot 3\cdot 3\cdot 10 - 2\cdot 2\cdot 1.5 = 949'

[area]
value = 585.4264068711929
exact = "600 − 30 − 30 − 2·4.5 + 30√2 (the chamfer, whose two triangular ends take 4.5 from each of the faces y = 0, 10) + 4·(2·1.5) (the pocket walls; its floor 2·2 replaces its mouth 2·2) = 543 + 30√2"
tex = '600 - 30 - 30 - 9 + 30\sqrt2 + 4\cdot(2\cdot 1.5) = 543 + 30\sqrt2'
+++

A cube of side 10 between the origin and (10, 10, 10). Chamfer the edge where its faces x = 10 and z = 10 meet by 3 on each face: the chamfer is the plane x + z = 17, a rectangle 10 long (along y) and 3√2 wide.

Then cut a square pocket into the chamfer face, square to it. The pocket's mouth is the square on the chamfer centred on the face's centre (8.5, 5, 8.5), 2 along y and 2 along the slope of the face (the direction (1, 0, −1)/√2). The pocket goes 1.5 deep into the cube, along the face's inward normal (−1, 0, −1)/√2, and has a flat floor parallel to the chamfer.

**As a set:**

$$\bigl([0,10]^3 \cap \{x + z \le 17\}\bigr) \setminus \{\,|y - 5| \le 1,\ |z - x|/\sqrt2 \le 1,\ -1.5 \le (x + z - 17)/\sqrt2 \le 0\,\}$$
