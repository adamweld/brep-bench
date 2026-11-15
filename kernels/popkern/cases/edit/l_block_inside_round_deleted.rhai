// An L-block's rounded inside corner deleted: the corner is sharp again.
let m = model();
let b = m.profile(xy()).path(0.0, 0.0).line_to(20.0, 0.0).line_to(20.0, 5.0).line_to(5.0, 5.0)
    .line_to(5.0, 15.0).line_to(0.0, 15.0).close().sketch("l").extrude(blind(8.0));
b = b.fillet(edges().straight().direction("z").at_vertex([5.0, 5.0, 0.0]), 2.0, #{ label: "round" });
b = b.delete_faces(faces().kind("cylinder"), #{ label: "round_deleted" });
m
