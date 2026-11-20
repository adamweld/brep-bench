"""A wedge grown by 1, its edges kept sharp. The wedge with its base the rectangle from (0, 0, 0) to
(10, 0, 6) in the plane y = 0, and its far face the smaller rectangle from (0, 8, 0) to (4, 8, 6)
in the plane y = 8. It has six flat faces: the base, the far face, the bottom z = 0, the top z =
6, the side x = 0, and a slanted face through the edges from (10, 0, 0) to (4, 8, 0) and from (10,
0, 6) to (4, 8, 6), which is the plane 8x + 6y = 80."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeWedge
from OCP.GeomAbs import GeomAbs_Intersection

from offset import offset

DISTANCE = 1


def build():
    wedge = BRepPrimAPI_MakeWedge(10, 8, 6, 4).Shape()  # base 10 by 6, far face 4 by 6, 8 apart
    return offset(wedge, DISTANCE, GeomAbs_Intersection)
