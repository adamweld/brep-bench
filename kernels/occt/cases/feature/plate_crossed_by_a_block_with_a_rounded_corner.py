"""A plate crossed by a sunk block with a rounded corner. A plate 10 thick: the quadrilateral with corners (0, 0), (40, 0), (34, 28) and (4, 22) in the xy
plane, from z = 0 to z = 10.

A block crosses its side from (40, 0) to (34, 28). Its footprint is the rectangle 26 ≤ x ≤ 44,
8 ≤ y ≤ 18 with its corner at (26, 18) rounded by a quarter circle of radius 4 about (30, 14), and it
runs from z = 6, 4 below the plate's top, up to z = 16, 6 above it. So it lies partly inside the
plate and partly out beyond the side, where its underside at z = 6 shows. The plate and the block
are one solid."""
from ribs import face, fuse, polygon, prism

S = 4 / 2 ** 0.5


def build():
    plate = prism(polygon([(0, 0, 0), (40, 0, 0), (34, 28, 0), (4, 22, 0)]), 0, 0, 10)
    # The block's footprint at z = 6: the rectangle with its (26, 18) corner rounded about (30, 14).
    footprint = face([((30, 18, 6), (44, 18, 6)), ((44, 18, 6), (44, 8, 6)), ((44, 8, 6), (26, 8, 6)),
                      ((26, 8, 6), (26, 14, 6)),
                      ((26, 14, 6), (30 - S, 14 + S, 6), (30, 18, 6))])
    block = prism(footprint, 0, 0, 10)
    return fuse(plate, block, "the block and the plate")
