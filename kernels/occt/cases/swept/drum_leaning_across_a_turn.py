"""A flat-end mill leaning across a turn, its rims digging past its ends. A ring with a channel cut
inside it by a drum leaning across a turn.

The ring is everything with distance from the z axis between 3 and 8 and with z between −2.5 and
2.5, all the way round.

The drum is a solid cylinder of radius 1 and length 3, its ends flat and square to its axis. Its
axis leans 25° from upright in the plane y = 1.2, along the direction (sin 25°, 0, cos 25°), and
its middle is the point (5, 1.2, 0).

It turns rigidly about the z axis, right-handed (from +x toward +y), by 100°, wholly inside the
ring. Cut from the ring everything the drum passes through.

The drum's side touches along two curves, each turned into a surface of revolution. Its rims are
sharp circles leaning to the turn's axis; each sweeps a ring whose section is no circle, and they
carry the cut past the flat ends' own contact lines. The channel is sealed inside the ring."""
from bench import Refused


def build():
    # The drum's axis, in the plane y = 1.2, runs past the turn's axis without meeting it. At
    # the point of its side at height s along its axis and angle t round it, the normal is
    # square to the turn's velocity where a relation in s and t holds that is no plane section,
    # no line and no circle of the cylinder. OCCT sweeps no solid, and a face it turns sweeps
    # to a solid only once split along its contact: finding that curve would be computing the
    # envelope outside the kernel.
    raise Refused("occt has no sweep of a solid: it turns faces, and the drum's side, whose "
                  "axis runs past the turn's without meeting it, touches what it sweeps along "
                  "curves that are no plane sections of it, which occt has no operation to find")
