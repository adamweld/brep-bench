// An L-block (foot (0..20, 0..10, 0..6), upright (0..6, 0..10, 0..20)): its concave edge rounded to
// 2, then the chain on the end face y = 0 that runs down the upright, round that round and along
// the foot rounded to 2.
let m = model();
let b = m.box(20.0, 10.0, 6.0);
b = b.union(m.tool(|m| m.box(6.0, 10.0, 20.0)));
b = b.fillet(edges().straight().direction("y").at_vertex([6.0, 0.0, 6.0]), 2.0, #{ label: "inside" });
let chain = edges().direction("z").at_vertex([6.0, 0.0, 20.0])
    .or(edges().direction("x").at_vertex([20.0, 0.0, 6.0]))
    .or(edges().circular().at_vertex([8.0, 0.0, 6.0]));
b = b.fillet(chain, 2.0, #{ label: "chain" });
m
