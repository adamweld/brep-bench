// The ellipse x²/25 + y²/9 ≤ 1 extruded from z = 0 to 10, offset by -1.0.
let m = model();
let e = m.profile(xy()).ellipse(5.0, 3.0).sketch("ellipse").extrude(blind(10.0));
e = e.offset_solid(-1.0, #{ label: "offset" });
m
