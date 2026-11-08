"""A coil spring pushed aslant through a plate. A plate, the box from (−15, −15, 0) to (15, 15, 4),
with a hole punched through it by a coil spring pushed aslant wholly through it. The tool is itself
a swept solid.

The spring is everything a ball of radius 0.4 passes through as its center runs along the helix of
radius 2 about the z axis that starts at (2, 0, −12), turns right-handed (from +x toward +y) and
rises 1.5 every turn, for three whole turns, ending at (2, 0, −7.5). Its coils are 1.5 apart and
0.8 thick, so they do not touch, and its ends are half balls.

It slides without turning along the vector (3, 1, 18), from wholly below the plate to wholly above
it. Cut from the plate everything the spring passes through.

Every level of the plate loses the spring's shadow along the push, a region bounded by curves the
coils' silhouettes draw, whose loops overlap one another."""
from bench import Refused


def build():
    # The spring itself OCCT can make, as the pipe of a circle along the helix with a ball at
    # each end. But pushed aslant, its coils touch what they sweep along the silhouette of the
    # pipe's surface seen along the push: a curve on that B-spline sheet with no closed form,
    # where its normal is square to the push. OCCT sweeps no solid, and the sheet slid whole
    # folds; split along that curve it would not, but OCCT has no operation that draws a
    # surface's silhouette as a curve on it (HLRBRep projects outlines for drawings), so
    # finding it would be computing the envelope outside the kernel.
    raise Refused("occt has no sweep of a solid: it slides faces, and the spring's pipe surface "
                  "touches what it sweeps along its silhouette seen along the push, a curve on "
                  "the B-spline sheet with no closed form, which occt has no operation to find")
