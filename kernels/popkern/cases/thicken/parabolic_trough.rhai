// The parabolic cylinder z = x²/8, -4 ≤ x ≤ 4, 0 ≤ y ≤ 10, as a B-spline sheet (quadratic in u
// on (-4, y, 2), (0, y, -2), (4, y, 2), linear in v), thickened 1 toward its focus.
let m = model();
let s = m.sheet_nurbs([[[-4.0, 0.0, 2.0], [-4.0, 10.0, 2.0]], [[0.0, 0.0, -2.0], [0.0, 10.0, -2.0]], [[4.0, 0.0, 2.0], [4.0, 10.0, 2.0]]],
    #{ degree: [2, 1], label: "trough" });
s = s.thicken(1.0, #{ side: "inward", label: "thick" });
m
