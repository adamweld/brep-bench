// The side ruled between two circles of radius 5 about z, at z = -5 and z = 5, each point of the
// lower joined to the point a quarter turn on round the upper, closed by the two discs: a loft
// between the circles, the upper's start a quarter turn on from the lower's.
let m = model();
let s = sqrt(0.5) * 5.0;
let lower = m.profile(xy_at(-5.0)).path(5.0, 0.0).arc_to([0.0, 5.0], [-5.0, 0.0]).arc_to([0.0, -5.0], [5.0, 0.0])
    .close().sketch("lower");
let upper = m.profile(xy_at(5.0)).path(0.0, 5.0).arc_to([-5.0, 0.0], [0.0, -5.0]).arc_to([5.0, 0.0], [0.0, 5.0])
    .close().sketch("upper");
m.loft([lower, upper], #{ label: "side" });
m
