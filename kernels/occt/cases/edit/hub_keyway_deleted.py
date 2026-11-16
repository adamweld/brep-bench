"""A hub (the tube about the z axis from z = 0 to 20, outer radius 10, bore 4) with a keyway along
its bore (the box −1 ≤ x ≤ 1, −5 ≤ y ≤ 0 cut away), the keyway's floor and walls deleted: the
bore closes over it.

The body is cut by BRepAlgoAPI_Cut, and BRepAlgoAPI_Defeaturing deletes the faces, picked by a
point inside each."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut

from bench import face_through
from edit import box, cylinder, deleted


def build():
    hub = BRepAlgoAPI_Cut(cylinder((0, 0, 0), 10, 20), cylinder((0, 0, -1), 4, 22)).Shape()
    body = BRepAlgoAPI_Cut(hub, box((-1, -5, -1), (1, 0, 21))).Shape()
    return deleted(body, [face_through(body, point) for point in [(0, -5, 10), (1, -4.5, 10), (-1, -4.5, 10)]])
