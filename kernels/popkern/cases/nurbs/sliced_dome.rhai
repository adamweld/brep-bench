// The block under the bicubic Bézier dome, cut by the plane z = 4 + x/2: what lies above it
// cut away.
let m = model();
let z = [[4.0, 6.0, 7.0, 5.0], [6.0, 14.0, 12.0, 7.0], [5.0, 13.0, 16.0, 8.0], [3.0, 6.0, 8.0, 6.0]];
let net = [];
for i in 0..4 {
    let row = [];
    for j in 0..4 { row.push([20.0 * i / 3.0, 20.0 * j / 3.0, z[i][j]]); }
    net.push(row);
}
let dome = m.sheet_nurbs(net, #{ degree: 3, label: "dome" });
// a block standing past the dome, split by it, the piece below kept
let block = m.box(20.0, 20.0, 30.0);
let b = block.split([dome], containing([10.0, 10.0, 1.0]))[0];
let above = m.tool(|m| m.profile(xz()).polygon([[-1.0, 3.5], [21.0, 14.5], [21.0, 40.0], [-1.0, 40.0]])
    .sketch("above").extrude(mid_plane(50.0)).translate(0.0, 10.0, 0.0));
b = b.subtract(above);
m
