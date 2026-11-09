+++
title = "The Utah teapot"
tags = ["integration", "part", "bezier", "nurbs", "surface", "sheet", "closed", "boolean", "union", "pole", "coincident", "coplanar", "point-contact"]
settled = false
origin = "Martin Newell's teapot (1975), the 32 patches with Frank Crow's bottom, in Newell's coordinates"
+++

Martin Newell's teapot made a solid: the 32 bicubic Bézier patches of the usual data set (Newell's
28 and the 4 of the bottom that Frank Crow added), in Newell's own coordinates, z up, not the
version squashed in z by Jim Blinn. The teapot rests on the origin, its spout toward +x and its
handle toward −x, and is symmetric about the plane y = 0.

The patches bound no solid by themselves: the lid stands apart from the rim, the spout and the
handle run into the body with their ends open, and the spout's mouth is open. So the solid is the
**union of four solids**, each bounded by some of the patches and closed by flat faces, given
below. It brings together what a kernel needs to take in a real free-form model: Bézier surfaces
from their control points, faces sewn into solids, surfaces that close to a point, flat faces
bounded by spline curves, and Booleans between free-form surfaces, with coincident faces and a
point of contact.

**A patch** is

$$S(u, v) = \sum_{i=0}^{3} \sum_{j=0}^{3} B_i(u)\, B_j(v)\, P_{ij}, \qquad 0 \le u, v \le 1,$$

over its 4 × 4 control points $P_{ij}$ (row $i$, column $j$, each from 0 to 3), with the cubic
Bernstein polynomials

$$B_0(t) = (1 - t)^3, \quad B_1(t) = 3t(1 - t)^2, \quad B_2(t) = 3t^2(1 - t), \quad B_3(t) = t^3.$$

**The rings.** The rim, the body, the bottom, the knob and the lid are rings of four patches. Each
ring is made from a profile of four points $(r_i, z_i)$. Its first patch, spanning the quarter
from +x to −y, has row $i$

$$P_{i0} = (r_i, 0, z_i), \quad P_{i1} = (r_i, -0.56\,r_i, z_i), \quad
P_{i2} = (0.56\,r_i, -r_i, z_i), \quad P_{i3} = (0, -r_i, z_i),$$

and its other three are that patch turned 90°, 180° and 270° about the z axis. The one exception
is the knob's row 1, which is (0.8, 0, 3.15), (0.8, −0.45, 3.15), (0.45, −0.8, 3.15),
(0, −0.8, 3.15): 0.45 where the rule gives 0.448. The profiles:

| ring | point 0 | point 1 | point 2 | point 3 |
|---|---|---|---|---|
| rim | (1.4, 2.4) | (1.3375, 2.53125) | (1.4375, 2.53125) | (1.5, 2.4) |
| upper body | (1.5, 2.4) | (1.75, 1.875) | (2, 1.35) | (2, 0.9) |
| lower body | (2, 0.9) | (2, 0.45) | (1.5, 0.225) | (1.5, 0.15) |
| bottom | (1.5, 0.15) | (1.5, 0.075) | (1.425, 0) | (0, 0) |
| knob | (0, 3.15) | (0.8, 3.15) | (0, 2.85) | (0.2, 2.7) |
| lid | (0.2, 2.7) | (0.4, 2.55) | (1.3, 2.55) | (1.3, 2.4) |

A row with $r = 0$ is one point on the axis: the bottom's patches close to a point at the origin,
the knob's at (0, 0, 3.15), and the knob's row 2 is the one point (0, 0, 2.85). The rings are not
surfaces of revolution: a quarter's Bézier curve is not a circle (it would need 0.5523, not 0.56),
so each section bulges by 0.4% at 45° between the axes. The rim is a rolled bead, from radius 1.4
up to about z = 2.5 and out to radius 1.5, where the body begins. The bottom is not flat: it is a
shallow dish touching the plane z = 0 at the origin alone.

**The spout and the handle** are each two patches on the side y ≤ 0 and their mirror images in
the plane y = 0. The patches on y ≤ 0, their control points $P_{ij}$ by row $i$ and column $j$:

| patch | row | column 0 | column 1 | column 2 | column 3 |
|---|---|---|---|---|---|
| spout, lower | 0 | (1.7, 0, 1.425) | (1.7, −0.66, 1.425) | (1.7, −0.66, 0.6) | (1.7, 0, 0.6) |
|  | 1 | (2.6, 0, 1.425) | (2.6, −0.66, 1.425) | (3.1, −0.66, 0.825) | (3.1, 0, 0.825) |
|  | 2 | (2.3, 0, 2.1) | (2.3, −0.25, 2.1) | (2.4, −0.25, 2.025) | (2.4, 0, 2.025) |
|  | 3 | (2.7, 0, 2.4) | (2.7, −0.25, 2.4) | (3.3, −0.25, 2.4) | (3.3, 0, 2.4) |
| spout, lip | 0 | (2.7, 0, 2.4) | (2.7, −0.25, 2.4) | (3.3, −0.25, 2.4) | (3.3, 0, 2.4) |
|  | 1 | (2.8, 0, 2.475) | (2.8, −0.25, 2.475) | (3.525, −0.25, 2.49375) | (3.525, 0, 2.49375) |
|  | 2 | (2.9, 0, 2.475) | (2.9, −0.15, 2.475) | (3.45, −0.15, 2.5125) | (3.45, 0, 2.5125) |
|  | 3 | (2.8, 0, 2.4) | (2.8, −0.15, 2.4) | (3.2, −0.15, 2.4) | (3.2, 0, 2.4) |
| handle, upper | 0 | (−1.6, 0, 2.025) | (−1.6, −0.3, 2.025) | (−1.5, −0.3, 2.25) | (−1.5, 0, 2.25) |
|  | 1 | (−2.3, 0, 2.025) | (−2.3, −0.3, 2.025) | (−2.5, −0.3, 2.25) | (−2.5, 0, 2.25) |
|  | 2 | (−2.7, 0, 2.025) | (−2.7, −0.3, 2.025) | (−3, −0.3, 2.25) | (−3, 0, 2.25) |
|  | 3 | (−2.7, 0, 1.8) | (−2.7, −0.3, 1.8) | (−3, −0.3, 1.8) | (−3, 0, 1.8) |
| handle, lower | 0 | (−2.7, 0, 1.8) | (−2.7, −0.3, 1.8) | (−3, −0.3, 1.8) | (−3, 0, 1.8) |
|  | 1 | (−2.7, 0, 1.575) | (−2.7, −0.3, 1.575) | (−3, −0.3, 1.35) | (−3, 0, 1.35) |
|  | 2 | (−2.5, 0, 1.125) | (−2.5, −0.3, 1.125) | (−2.65, −0.3, 0.9375) | (−2.65, 0, 0.9375) |
|  | 3 | (−2, 0, 0.9) | (−2, −0.3, 0.9) | (−1.9, −0.3, 0.6) | (−1.9, 0, 0.6) |

Each patch's columns 0 and 3 lie in the plane y = 0, where it meets its mirror image. Each end
of the spout and of the handle is a closed loop of two cubic Bézier curves, a row of a patch and
the same row of its mirror image, and every such loop lies in a plane.

**The four solids:**

1. **The body**, bounded by the rim, the upper and lower body and the bottom (16 patches), and
   closed by the flat disk inside the rim's inner edge (the rim's row 0, radius 1.4) in the plane
   z = 2.4.
2. **The lid**, bounded by the knob and the lid (8 patches), and closed by the flat disk inside
   the lid's edge (the lid's row 3, radius 1.3) in the plane z = 2.4. It lies on the body's disk.
3. **The spout**, bounded by its 4 patches, and closed at its base by the flat face inside the
   loop of the lower patches' row 0, in the plane x = 1.7, and at its mouth by the flat face
   inside the loop of the lip's row 3, in the plane z = 2.4 (from x = 2.8 to x = 3.2, ringed by
   the rolled lip). So the spout is solid to its mouth.
4. **The handle**, bounded by its 4 patches, and closed at each end by the flat face inside that
   end's loop: the upper patches' row 0, in the plane through (−1.6, 0, 2.025) and
   (−1.5, 0, 2.25) parallel to the y axis, and the lower patches' row 3, in the plane through
   (−2, 0, 0.9) and (−1.9, 0, 0.6) parallel to the y axis.

**The union.** The spout's base and the handle's upper end lie inside the body, so their flat
faces are swallowed. The handle's lower end lies inside the body too, but for one point,
(−2, 0, 0.9): there its loop touches the body's widest circle, where the body stands upright at
radius 2, and the curve along which the handle runs into the body passes through it. The lid's
disk lies on the body's, so they join over it, and the ring of the body's disk between radius 1.3
and 1.4, the gap between the lid and the rim, stays a face of the solid. The teapot is one solid
with no hole through it but the one the handle makes.

The four solids' volumes are exact (by the divergence theorem, the integrand on each patch being a
polynomial): the body 42702346587/1750000000 ≈ 24.4013409069, the lid 743579407/875000000 ≈
0.8498050366, the spout 2623953/4000000 = 0.65598825, and the handle 13149/50000 = 0.26298. The
union's volume is their sum less the parts of the spout and the handle inside the body, and those,
like every area here, have no closed form, so the case is unsettled until two kernels agree.
