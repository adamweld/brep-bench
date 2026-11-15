// The solid under the Coons patch of the skew quadrilateral (0, 0, 5), (10, 0, 5), (10, 10, 15),
// (0, 10, 5): with straight edges the patch is the surface ruled between its edges on x = 0 and
// x = 10, z = 5 + xy/10. So the solid is the loft between its faces on x = 0 and x = 10, a
// rectangle and a trapezoid, corner to corner.
let m = model();
let near = m.profile(yz()).polygon([[0.0, 0.0], [10.0, 0.0], [10.0, 5.0], [0.0, 5.0]]).sketch("x0");
let far = m.profile(yz_at(10.0)).polygon([[0.0, 0.0], [10.0, 0.0], [10.0, 15.0], [0.0, 5.0]]).sketch("x10");
m.loft([near, far], #{ label: "under_patch" });
m
