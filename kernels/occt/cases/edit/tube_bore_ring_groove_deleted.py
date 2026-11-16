"""A tube about the z axis from z = 0 to z = 20, outer radius 10, bore radius 4, with a ring groove
turned into the bore (everything inside radius 6 from z = 8 to z = 12 cut away), the groove's
floor and both walls deleted together: the bore below and above the groove is one surface, which
runs on across it.

The groove is cut away by BRepAlgoAPI_Cut, and BRepAlgoAPI_Defeaturing deletes its faces, picked
by a point inside each."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut

from bench import face_through
from edit import cylinder, deleted


def build():
    tube = BRepAlgoAPI_Cut(cylinder((0, 0, 0), 10, 20), cylinder((0, 0, -1), 4, 22)).Shape()
    grooved = BRepAlgoAPI_Cut(tube, cylinder((0, 0, 8), 6, 4)).Shape()
    groove = [face_through(grooved, point) for point in [(0, -6, 10), (0, -5, 8), (0, -5, 12)]]
    return deleted(grooved, groove)
