"""A skewed block with a corner sliced off. With u = x + y/4, v = y + z/4, w = z + x/4, the set
where u, v, w each lie between 0 and 6 and x + y + z ≤ 12."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from halfspace import keep_below


def build():
    shape = BRepPrimAPI_MakeBox(gp_Pnt(-10, -10, -10), 30, 30, 30).Shape()
    # Each form f = n·p lies between 0 and 6: keep f ≥ 0, then keep f ≤ 6.
    for n in ((1, 0.25, 0), (0, 1, 0.25), (0.25, 0, 1)):
        shape = keep_below(shape, (0, 0, 0), tuple(-c for c in n))
        s = 6 / sum(c * c for c in n)  # the point s·n has n·p = 6
        shape = keep_below(shape, tuple(s * c for c in n), n)
    return keep_below(shape, (12, 0, 0), (1, 1, 1))
