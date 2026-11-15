// The pyramid on the rectangle (-10, -6) to (10, 6) with its apex at (0, 0, 12), all eight edges
// rounded to radius 3/2. The pyramid is a block less the four slabs beyond its sloping faces.
let m = model();
let p = m.box(20.0, 12.0, 12.0, #{ at: [-10.0, -6.0, 0.0] });
for s in [1.0, -1.0] {
    // beyond the faces through the long sides, y = ±(6 - z/2)
    p = p.subtract(m.tool(|m| m.profile(yz()).polygon([[s * 6.5, -1.0], [s * 7.0, -1.0], [s * 7.0, 13.0], [s * -0.5, 13.0]])
        .sketch("long_side").extrude(mid_plane(30.0))));
    // beyond the faces through the short ends, x = ±(10 - 5z/6)
    p = p.subtract(m.tool(|m| m.profile(xz()).polygon([[s * (10.0 + 5.0 / 6.0), -1.0], [s * 11.0, -1.0], [s * 11.0, 13.0], [s * (10.0 - 65.0 / 6.0), 13.0]])
        .sketch("short_end").extrude(mid_plane(30.0))));
}
p = p.fillet(edges(), 1.5, #{ label: "rounds" });
m
