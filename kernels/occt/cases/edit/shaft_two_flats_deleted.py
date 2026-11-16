"""A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with two opposite flats
(everything with |x| > 4 cut away), both flats deleted together: the cylinder closes over them.

The body is cut by BRepAlgoAPI_Cut, and BRepAlgoAPI_Defeaturing deletes the faces, picked by a
point inside each."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut

from bench import face_through
from edit import box, cylinder, deleted


def build():
    one = BRepAlgoAPI_Cut(cylinder((0, 0, 0), 5, 20), box((4, -6, -1), (6, 6, 21))).Shape()
    body = BRepAlgoAPI_Cut(one, box((-6, -6, -1), (-4, 6, 21))).Shape()
    return deleted(body, [face_through(body, (4, 0, 10)), face_through(body, (-4, 0, 10))])
