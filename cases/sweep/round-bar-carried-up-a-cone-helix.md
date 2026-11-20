+++
title = "A round bar carried up a helix that widens as it climbs"
tags = ["sweep", "helix", "cone", "torus", "unsettled"]
origin = "FreeCAD src/Mod/PartDesign/PartDesignTests/TestHelix.py::testCircleQ1"
settled = false
+++

A bar of circular section carried up a helix about the z axis whose radius grows as it climbs, a coil that opens like a cone.

The path c(θ) starts at (5, 0, 0) and turns right-handed (from +x toward +y) through three whole turns: for each turn it rises 4 and its distance from the axis grows by 1, so c(θ) = (ρ cos θ, ρ sin θ, z) with ρ = 5 + θ/360° and z = 4 θ/360°, for 0 ≤ θ ≤ 1080°. It lies on a cone about the z axis whose side leans at arctan(1/4) to the axis.

The section is the disc of radius 1 about (5, 0, 0) in the plane y = 0.

Each section is carried along the path by the path's own frame, with the z axis held in it: at each point of the path, T is the unit tangent, B is the z axis with its component along T taken out (made unit), and N = B × T. The section keeps its coordinates in (T, N, B): a point p of the starting section goes to c(θ) + R(θ)(p − c(0)), where c(θ) is the point of the path and R(θ) the rotation taking the starting frame to the frame at c(θ). On a cylinder this is a plain screw and the section stays in a plane through the axis; on the cone the path's tangent leans outward as it climbs, so the section leans with it, and does not stay in a plane through the axis.

The bar's ends are flat discs. Its turns are 4 apart in height and 1 apart in radius while the disc is 2 wide, so they do not touch. It has no closed form.

**As a set:**

$$\bigcup_{t \in [0,\,3]} \bigl\{\, c(360^\circ t) + R(360^\circ t)\,(p - c(0)) : p \in Q \,\bigr\}$$

Q is the disc, and R the rotation of the frame given above.
