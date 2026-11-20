"""A cube edge rounded with a radius that falls to nothing at its middle and grows again. A cube of side 10 between the origin and (10, 10, 10). Round the edge where its faces x = 10 and
y = 10 meet, the line from (10, 10, 0) to (10, 10, 10), with a radius that falls to nothing in the
middle of the edge and grows again: r = 2(1 − z/5)², so 2 at the bottom face z = 0, 0 at the middle
z = 5 and 2 at the top face z = 10. At the middle the round has no width and the edge is sharp
there; the round is two horns, one from each end, meeting at that one point.

The round's sections square to the edge are circular: at each height z the round's section by the
horizontal plane is the quarter circle of radius r(z) centered at (10 − r, 10 − r, z), tangent to
the face x = 10 at (10, 10 − r, z) and to the face y = 10 at (10 − r, 10, z). It cuts the bottom and
top faces each with the quarter circle of radius 2."""
from OCP.BRepPrimAPI import BRepPrimAPI_MakeBox

from blend import edges_where
from vround import rounded_with_law

SIDE = 10
TOLERANCE = 1e-7


def build():
    cube = BRepPrimAPI_MakeBox(SIDE, SIDE, SIDE).Shape()
    [edge] = edges_where(cube, lambda p: abs(p.X() - SIDE) < TOLERANCE and abs(p.Y() - SIDE) < TOLERANCE)
    # r = 2(1 − z/5)², set at stations along the edge: 0 at the middle one.
    return rounded_with_law(cube, edge, lambda p: 2 * (1 - p.Z() / 5) ** 2,
                            "the edge down to a radius of 0 at its middle")
