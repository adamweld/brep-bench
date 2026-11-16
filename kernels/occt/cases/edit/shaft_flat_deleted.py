"""A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a flat milled along
its whole length (everything with y < −4 cut away), the flat deleted: the shaft's cylinder, on
both its long sides, closes over it.

The box is cut away by BRepAlgoAPI_Cut, and BRepAlgoAPI_Defeaturing deletes the faces, picked by a
point inside each."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut

from bench import face_through
from edit import box, cylinder, deleted


def build():
    body = BRepAlgoAPI_Cut(cylinder((0, 0, 0), 5, 20), box((-6, -6, -1), (6, -4, 21))).Shape()
    return deleted(body, [face_through(body, point) for point in [(0, -4, 10)]])
