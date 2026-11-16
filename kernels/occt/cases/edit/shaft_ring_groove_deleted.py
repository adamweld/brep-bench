"""A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a ring groove
turned into it (everything between radius 4 and 5 from z = 8 to z = 12 cut away), the groove's
floor and both walls deleted together: the shaft's cylinder below and above the groove is one
surface, which runs on across it.

The ring is cut away by BRepAlgoAPI_Cut, and BRepAlgoAPI_Defeaturing deletes the groove's three
faces, picked by a point inside each."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut

from bench import face_through
from edit import cylinder, deleted


def build():
    ring = BRepAlgoAPI_Cut(cylinder((0, 0, 8), 7, 4), cylinder((0, 0, 7), 4, 6)).Shape()
    grooved = BRepAlgoAPI_Cut(cylinder((0, 0, 0), 5, 20), ring).Shape()
    groove = [face_through(grooved, point) for point in [(0, -4, 10), (0, -4.5, 8), (0, -4.1, 12)]]
    return deleted(grooved, groove)
