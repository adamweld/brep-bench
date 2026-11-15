fn ball(m, r) {
    m.sphere(r)
}

// Two balls of radius 5, on the origin and on (9.999999, 0, 0), overlapping 1e-6 deep: their
// common part.
// The lens is 1e-6 thick, popkern's default linear tolerance: under it a body has no inside, so
// the part declares the tolerance its features need.
let m = model(#{ tolerance: 1e-8 });
let lens = ball(m, 5.0);
let other = m.tool(|m| ball(m, 5.0).translate(9.999999, 0.0, 0.0));
lens = lens.intersect(other);
m
