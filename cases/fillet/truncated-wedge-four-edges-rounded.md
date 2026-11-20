+++
title = "A truncated wedge with its four edges along its depth rounded, each to its own radius"
tags = ["fillet", "planar", "cylinder", "analytic"]
origin = "OCCT tests/blend/tolblend_simple/A1, OCCT tests/blend/tolblend_simple/A2, OCCT tests/blend/tolblend_simple/D3, OCCT tests/blend/tolblend_simple/D4, OCCT tests/blend/tolblend_simple/D5"

[volume]
value = 1241.4219582585940
exact = "8A, A = 160 − Σ rᵢ²(cot(θᵢ/2) − (π − θᵢ)/2) over the quadrilateral’s four corners (angles 90°, 51.34° with cos = 8/√164, 128.66°, 90°; radii 1, 2, 3, 1.5) = 155.17774478232424"
tex = '8\Bigl(160 - \sum_{i} r_i^2\bigl(\cot\tfrac{\theta_i}{2} - \tfrac{\pi - \theta_i}{2}\bigr)\Bigr) = 8 \cdot 155.17774478232424'

[area]
value = 708.00534163455386
exact = "2·155.17774478232424 (the two flat ends) + 8·(42 + √164 − Σ(2rᵢ cot(θᵢ/2) − rᵢ(π − θᵢ))) (the sides: the four flats and the four arcs), the rounded quadrilateral’s perimeter being 49.706231508738172"
tex = '2\cdot 155.17774478232424 + 8\Bigl(42 + \sqrt{164} - \sum_i\bigl(2r_i\cot\tfrac{\theta_i}{2} - r_i(\pi - \theta_i)\bigr)\Bigr) = 2\cdot 155.17774478232424 + 8\cdot 49.706231508738172'
+++

A truncated wedge: the prism on the quadrilateral through (0, 0), (20, 0), (12, 10) and (0, 10) in
the xy plane, from z = 0 to z = 8. It has a flat bottom 20 wide, a flat top 12 wide, an upright face
x = 0, a slanting face from (20, 0) to (12, 10), and two flat ends z = 0 and z = 8.

Round the four edges along z, the ones through the quadrilateral's four corners, each to a radius of
its own: 1 at (0, 0), 2 at (20, 0), 3 at (12, 10) and 1.5 at (0, 10). Each round is a cylinder, its
axis along z, tangent to the two faces of its edge, and it runs the whole depth, from one flat end to
the other. No two of these edges share an end, so no round meets another: the quadrilateral's corners
are rounded in the plane and the result is that quadrilateral extruded. The corners' angles are 90°
at (0, 0) and (0, 10), 51.34° at (20, 0) and 128.66° at (12, 10).
