// A dumbbell plate (two 10 by 10 squares joined by a neck 4 long and 2 wide, 6 tall) shrunk by
// 1.5: the neck goes, leaving two bodies.
let m = model();
let d = m.profile(xy()).polygon([[0.0, 0.0], [10.0, 0.0], [10.0, 4.0], [14.0, 4.0], [14.0, 0.0],
    [24.0, 0.0], [24.0, 10.0], [14.0, 10.0], [14.0, 6.0], [10.0, 6.0], [10.0, 10.0], [0.0, 10.0]])
    .sketch("outline").extrude(blind(6.0));
d = d.offset_solid(-1.5, #{ label: "shrunk" });
m
