"""A ridge poking through a block's leaning side. A block whose side leans: everything with 0 ≤ y ≤ 30, 0 ≤ z ≤ 20 and 0 ≤ x ≤ 40 − z/5. Its side
x = 40 − z/5 leans in as it rises, from x = 40 at the bottom to x = 36 at the top; its top face is
the rectangle 0 ≤ x ≤ 36, 0 ≤ y ≤ 30 at z = 20.

A ridge runs along the block's top over the leaning side. Its section, in planes perpendicular to x,
is the triangle with corners (y, z) = (9, 18), (21, 18) and (15, 26): a roof 12 wide and 8 high, whose
foot lies 2 below the top face. It is swept along x from x = 10 to x = 44, so its two ends are flat,
in the planes x = 10 and x = 44. The ridge's two sloping faces run over the top face and past the
leaning side, which they cross between z = 18 and z = 20; beyond the side the ridge's flat underside
at z = 18 shows, from x = 36.4 to 44. The block and the ridge are one solid."""
from ribs import fuse, polygon, prism


def build():
    # The block: its section in the plane y = 0, the side x = 40 - z / 5 leaning in, run along y to 30.
    block = prism(polygon([(0, 0, 0), (40, 0, 0), (36, 0, 20), (0, 0, 20)]), 0, 30, 0)
    # The ridge: the triangle (y, z) = (9, 18), (21, 18), (15, 26) at x = 10, run along x to 44.
    ridge = prism(polygon([(10, 9, 18), (10, 21, 18), (10, 15, 26)]), 34, 0, 0)
    return fuse(block, ridge, "the ridge and the block")
