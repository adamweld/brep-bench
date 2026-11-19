"""A cube edge rounded with a radius falling to nothing at the corner. A cube of side 10 between the
origin and (10, 10, 10). Round the edge where its faces x = 10 and y = 10 meet, the line from
(10, 10, 0) to (10, 10, 10), with a radius that falls linearly along it to nothing: r = 2 − z/5, so
2 at the bottom face z = 0 and 0 at the top face z = 10.

The round's sections square to the edge are circular: at each height z the round's section by the
horizontal plane is the quarter circle of radius r(z) centred at (10 − r, 10 − r, z), tangent to the
face x = 10 at (10, 10 − r, z) and to the face y = 10 at (10 − r, 10, z). So the round is part of an
oblique circular cone whose apex is the cube's corner (10, 10, 10), tangent to both faces along
straight lines from (10, 8, 0) and (8, 10, 0) to that corner. It cuts the bottom face with the
quarter circle of radius 2 and leaves the top face whole: the round, the two faces and the top all
meet at the one corner."""
from OCP.BRep import BRep_Tool
from OCP.BRepFilletAPI import BRepFilletAPI_MakeFillet
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox
from OCP.TopExp import TopExp

from bench import Refused
from blend import edges_where

SIDE = 10
RADIUS_BOTTOM, RADIUS_TOP = 2, 0  # at z = 0 and z = 10
TOLERANCE = 1e-7


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    [edge] = edges_where(cube, lambda p: abs(p.X() - SIDE) < TOLERANCE
                         and abs(p.Y() - SIDE) < TOLERANCE)
    fillet = BRepFilletAPI_MakeFillet(cube)
    # A radius from the spine's first vertex to its last: give each end its own.
    if BRep_Tool.Pnt_s(TopExp.FirstVertex_s(edge)).Z() < SIDE / 2:
        fillet.Add(RADIUS_BOTTOM, RADIUS_TOP, edge)
    else:
        fillet.Add(RADIUS_TOP, RADIUS_BOTTOM, edge)
    fillet.Build()
    if not fillet.IsDone():
        raise Refused("BRepFilletAPI_MakeFillet could not round the edge down to a radius of 0")
    return fillet.Shape()
