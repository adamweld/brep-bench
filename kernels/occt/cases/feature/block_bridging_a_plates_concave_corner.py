"""A block with a rounded corner bridging a plate's concave corner. A plate 10 thick with a concave corner: the pentagon with corners (0, 0), (40, 0), (40, 14), (18, 22)
and (0, 30) in the xy plane, from z = 0 to z = 10. Its sides from (40, 14) to (18, 22) and from
(18, 22) to (0, 30) meet at the concave corner (18, 22).

A block bridges that corner. Its footprint is the rectangle 10 ≤ x ≤ 28, 12 ≤ y ≤ 30 with its corner
at (28, 30) rounded by a quarter circle of radius 4 about (24, 26), and it runs from z = 6, 4 below
the plate's top, up to z = 16, 6 above it. It covers the corner and fills out beyond the two sides,
where its underside at z = 6 shows; its rounded corner lies wholly outside the plate. The plate and
the block are one solid."""
from ribs import face, fuse, polygon, prism

S = 4 / 2 ** 0.5


def build():
    plate = prism(polygon([(0, 0, 0), (40, 0, 0), (40, 14, 0), (18, 22, 0), (0, 30, 0)]), 0, 0, 10)
    # The block's footprint at z = 6: the rectangle with its (28, 30) corner rounded about (24, 26).
    footprint = face([((10, 30, 6), (24, 30, 6)),
                      ((24, 30, 6), (24 + S, 26 + S, 6), (28, 26, 6)),
                      ((28, 26, 6), (28, 12, 6)), ((28, 12, 6), (10, 12, 6)), ((10, 12, 6), (10, 30, 6))])
    block = prism(footprint, 0, 0, 10)
    return fuse(plate, block, "the block and the plate")
