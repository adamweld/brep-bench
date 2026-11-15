// A block on the square 0..20 by 0..20 under a Bézier patch of degree 15 each way: control points
// (20i/15, 20j/15, 4 + ((3i + 5j) mod 7)).
let m = model();
let net = [];
for i in 0..16 {
    let row = [];
    for j in 0..16 { row.push([20.0 * i / 15.0, 20.0 * j / 15.0, 4.0 + ((3 * i + 5 * j) % 7).to_float()]); }
    net.push(row);
}
let dome = m.sheet_nurbs(net, #{ degree: 15, label: "dome" });
let block = m.box(20.0, 20.0, 30.0);
let b = block.split([dome], containing([10.0, 10.0, 1.0]))[0];
m
