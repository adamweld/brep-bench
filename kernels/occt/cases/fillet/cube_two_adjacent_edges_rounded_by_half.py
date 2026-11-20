"""A cube with two adjacent edges rounded by half its side. A cube of side 10 between the origin
and (10, 10, 10). Round its two top edges that meet at the corner (10, 10, 10), the edge along x
where y = 10 and z = 10 meet and the edge along y where x = 10 and z = 10 meet, to radius 5."""
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from bench import Refused
from chamfer import edge_through

RADIUS = 5


def build():
    cube = BRepPrimAPI_MakeBox(10, 10, 10).Shape()
    fillet = BRepFilletAPI_MakeFillet(cube)
    fillet.Add(RADIUS, edge_through(cube, (5, 10, 10)))  # along x at y = 10, z = 10
    fillet.Add(RADIUS, edge_through(cube, (10, 5, 10)))  # along y at x = 10, z = 10
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the two edges")
    return fillet.Shape()
