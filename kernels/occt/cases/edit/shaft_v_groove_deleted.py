"""A shaft, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a V-groove turned
into it (everything farther from the axis than 3 + |z − 10| cut away: two cones meeting at radius
3 at z = 10), both cones deleted together: the shaft's cylinder below and above the groove is one
surface, which runs on across it.

The groove is cut away by BRepAlgoAPI_Cut, and BRepAlgoAPI_Defeaturing deletes its faces, picked
by a point inside each."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut

from bench import face_through
from edit import cylinder, deleted, frustum


def build():
    lower = BRepAlgoAPI_Cut(cylinder((0, 0, 7), 7, 3), frustum((0, 0, 7), 6, 3, 3)).Shape()
    upper = BRepAlgoAPI_Cut(cylinder((0, 0, 10), 7, 3), frustum((0, 0, 10), 3, 6, 3)).Shape()
    half = BRepAlgoAPI_Cut(cylinder((0, 0, 0), 5, 20), lower).Shape()
    grooved = BRepAlgoAPI_Cut(half, upper).Shape()
    groove = [face_through(grooved, point) for point in [(0, -4, 9), (0, -4, 11)]]
    return deleted(grooved, groove)
