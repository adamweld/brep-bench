"""A triangular prism lofted between two triangles, shelled with its top open. The prism made by a
ruled loft between the triangle (0, 0, 0), (6, 0, 0), (0, 8, 0) and the equal triangle at z = 10,
closed by the two triangles, shelled inward to a wall 1 thick, open at its top face."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakePolygon
from OCP.BRepOffsetAPI import BRepOffsetAPI_ThruSections
from OCP.GeomAbs import GeomAbs_Intersection
from OCP.gp import gp_Pnt

from bench import Refused
from shell import face_through, shell

HEIGHT = 10
THICKNESS = 1


def triangle(z):
    polygon = BRepBuilderAPI_MakePolygon()
    for x, y in ((0, 0), (6, 0), (0, 8)):
        polygon.Add(gp_Pnt(x, y, z))
    polygon.Close()
    return polygon.Wire()


def build():
    loft = BRepOffsetAPI_ThruSections(True, True)  # a solid, ruled between the sections
    loft.AddWire(triangle(0))
    loft.AddWire(triangle(HEIGHT))
    loft.Build()
    if not loft.IsDone():
        raise Refused("BRepOffsetAPI_ThruSections could not loft the prism")
    prism = loft.Shape()
    return shell(prism, [face_through(prism, (2, 2, HEIGHT))], THICKNESS,
                 join=GeomAbs_Intersection)
