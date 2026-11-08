"""A tilted box pushed aslant wholly through a plate. A plate, the box from (−15, −15, 0) to (15,
15, 4), with a hole punched through it by a tilted box pushed aslant wholly through it.

The box is 4 × 3 × 2, its edges along the orthonormal directions e₁ = (2, 1, 2)/3, e₂ = (−2, 2,
1)/3 and e₃ = (1, 2, −2)/3: everything (−2, −1, −8) + a e₁ + b e₂ + c e₃ with |a| ≤ 2, |b| ≤ 1.5
and |c| ≤ 1. As placed it is wholly below the plate.

It slides without turning along the vector (4, 2, 18), ending centered on (2, 1, 10), wholly above
the plate. Cut from the plate everything the box passes through.

No face or edge of the box is level or upright: where it passes, its silhouette turns at its
corners. Each level of the plate loses the box's whole shadow along the push, a hexagon, so the
hole is an oblique hexagonal prism."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from sweep import cut, slid

PLATE = ((-15, -15, 0), (15, 15, 4))
CENTRE = (-2, -1, -8)
E1, E2, E3 = (2 / 3, 1 / 3, 2 / 3), (-2 / 3, 2 / 3, 1 / 3), (1 / 3, 2 / 3, -2 / 3)
HALF = (2, 1.5, 1)  # half the box's size along e₁, e₂ and e₃
PUSH = (4, 2, 18)


def build():
    # e₁ × e₂ = −e₃, so the box's frame is e₁, e₂, −e₃, with its corner at the centre less half
    # of each side
    corner = [c - HALF[0] * a - HALF[1] * b + HALF[2] * d
              for c, a, b, d in zip(CENTRE, E1, E2, E3)]
    frame = gp_Ax2(gp_Pnt(*corner), gp_Dir(*(-d for d in E3)), gp_Dir(*E1))
    box = BRepPrimAPI_MakeBox(frame, 2 * HALF[0], 2 * HALF[1], 2 * HALF[2]).Shape()
    # Every face is flat and none is parallel to the push: none needs splitting.
    swept = slid(box, PUSH)
    return cut(BRepPrimAPI_MakeBox(gp_Pnt(*PLATE[0]), gp_Pnt(*PLATE[1])).Shape(), swept)
