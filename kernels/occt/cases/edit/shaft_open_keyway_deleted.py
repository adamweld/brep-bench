"""A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with an open keyway (the
box −1 ≤ x ≤ 1, y ≤ −3, z ≥ 8 cut away: 2 wide, from z = 8 out through the top end), its floor,
two walls and end deleted together: the shaft's cylinder and top close over it.

The box is cut away by BRepAlgoAPI_Cut, and BRepAlgoAPI_Defeaturing deletes the faces, picked by a
point inside each."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut

from bench import face_through
from edit import box, cylinder, deleted


def build():
    body = BRepAlgoAPI_Cut(cylinder((0, 0, 0), 5, 20), box((-1, -6, 8), (1, -3, 21))).Shape()
    return deleted(body, [face_through(body, point) for point in [(0, -3, 15), (1, -4, 15), (-1, -4, 15), (0, -4, 8)]])
