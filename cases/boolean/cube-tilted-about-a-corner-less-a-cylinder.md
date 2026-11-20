+++
title = "A cube tilted about a corner below a cylinder's base, less the cylinder"
tags = ["boolean", "cut", "general-position", "planar", "cylinder"]
settled = false
origin = "OCCT tests/boolean/boptuc_simple/ZC5"
+++

A cylinder of radius 3 and height 6 standing on the xy plane with its axis the z axis, and a cube of edge 4 with one corner at P = (0, 0, −1) on the axis, just below the cylinder's base. Start with the cube's three edges from P running along +x, +y and +z; then turn the cube 30° about the horizontal line through P in the direction (1, 1, 0), right-handed. The line is the diagonal of the cube's lower face through P, and the turn carries the cube's +z edge toward +x and −y, so that the cube's far corners rise through the base disc. Cut the cylinder away from the cube.

The cube straddles the plane z = 0: its faces cut the base disc and the wall, and the cylinder's top, at z = 6, is untouched. What is left is the part of the cube outside the cylinder: the part below the plane z = 0 and the part beyond the wall. The cube's faces meet the wall in arcs of ellipses, so the case has no closed form and is unsettled.
