"""An oblique triangular frustum shelled with its top open. Lofted straight between the triangle
(0, 0, 0), (12, 0, 0), (0, 9, 0) in z = 0 and the triangle (2, 1, 8), (8, 1, 8), (2, 11/2, 8) in
z = 8, shelled inward to a wall 1 thick, open at its top face."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.gp import gp_Pnt

from bench import Refused
from shell import face_through, shell

BOTTOM = [(0, 0, 0), (12, 0, 0), (0, 9, 0)]
TOP = [(2, 1, 8), (8, 1, 8), (2, 5.5, 8)]


def triangle(corners):
    polygon = BRepBuilderAPI_MakePolygon()
    for corner in corners:
        polygon.Add(gp_Pnt(*corner))
    polygon.Close()
    return polygon.Wire()


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled
    loft.AddWire(triangle(BOTTOM))
    loft.AddWire(triangle(TOP))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections declined the frustum")
    frustum = loft.Shape()
    return shell(frustum, [face_through(frustum, (4, 2, 8))], 1)
