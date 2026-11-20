"""A square bar cut aslant at both ends, one edge chamfered. Bounded by y = 0, y = 6, z = 0,
z = 6, the plane y + z = 10, the left end x = 0.4z and the right end x = 20 − 0.5y."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.gp import gp_Pnt

from halfspace import keep_below


def build():
    bar = BRepPrimAPI_MakeBox(gp_Pnt(-1, 0, 0), 22, 6, 6).Shape()
    bar = keep_below(bar, (0, 0, 0), (-1, 0, 0.4))      # x ≥ 0.4z
    bar = keep_below(bar, (20, 0, 0), (1, 0.5, 0))      # x ≤ 20 − 0.5y
    return keep_below(bar, (0, 6, 4), (0, 1, 1))        # y + z ≤ 10
