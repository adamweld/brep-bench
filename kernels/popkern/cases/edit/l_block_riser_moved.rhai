// An L-block (section (0, 0), (40, 0), (40, 10), (15, 10), (15, 30), (0, 30) in y = 0, 20 deep);
// then its riser, the face x = 15, moved out 10 along +x, its neighbours following.
let m = model();
let b = m.profile(xz()).polygon([[0.0, 0.0], [40.0, 0.0], [40.0, 10.0], [15.0, 10.0], [15.0, 30.0], [0.0, 30.0]])
    .sketch("l").extrude(blind(-20.0));
b = b.offset_faces(faces().facing("x").except(faces().facing("x").at_height(5.0)).except(faces().facing("x").at_height(15.0)), 10.0, #{ label: "riser_moved" });
m
