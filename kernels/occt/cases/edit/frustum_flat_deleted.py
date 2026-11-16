"""A cone frustum about the z axis, radius 6 at z = 0 to 3 at z = 20, with a flat milled along its
side (everything with y < −4 cut away), the flat deleted: the cone closes over it.

The body is cut by BRepAlgoAPI_Cut, and BRepAlgoAPI_Defeaturing deletes the faces, picked by a
point inside each."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Cut

from bench import face_through
from edit import box, deleted, frustum


def build():
    body = BRepAlgoAPI_Cut(frustum((0, 0, 0), 6, 3, 20), box((-7, -7, -1), (7, -4, 21))).Shape()
    return deleted(body, [face_through(body, (0, -4, 5))])
