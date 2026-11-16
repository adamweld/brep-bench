"""A filleted cone boss grown to twice its fillet's radius. A plate, the cylinder of radius 8 about the z axis from z = −2 to z = 0, and on it a boss, the cone frustum from
radius 4 at z = 0 to radius 1 at z = 6, joined. The concave edge where the boss's cone meets the plate's top
is filleted to radius 1.5: a piece of a torus tangent to the top and to the cone. Grow the solid by 3 with
rounded edges: every point within 3 of it.

That is more than the fillet's radius. A fillet grown by its radius or more shrinks to nothing: its
moved face would be a torus of tube radius 1.5 − 3 ≤ 0, and the moved top and the moved cone run on until they meet
in a sharp concave edge, as if the fillet had never been there. So the result is the sharp plate and boss
grown by 3: the bottom moved down 3, the plate's side out 3 with a quarter round of radius 3 at each of its
rims, the plate's top up 3 and the cone out 3 along its normal, meeting in the circle of radius 5.85410196625 at
z = 3, and the boss's top up 3 with a round of radius 3 where it meets the cone."""
from OCP.BRepAlgoAPI import BRepAlgoAPI_Fuse
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeSolid
from OCP.BRepPrimAPI import BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
from OCP.TopAbs import TopAbs_ShapeEnum
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Ax2, gp_Dir, gp_Pnt

from bench import Refused
from blend import edges_where, rounded
from edit import on_circle
from offset import offset

DISTANCE = 3.0


def build():
    up = gp_Dir(0, 0, 1)
    plate = BRepPrimAPI_MakeCylinder(gp_Ax2(gp_Pnt(0, 0, -2), up), 8, 2).Shape()
    joined = BRepAlgoAPI_Fuse(plate, BRepPrimAPI_MakeCone(4, 1, 6).Shape())
    if not joined.IsDone():
        raise Refused("BRepAlgoAPI_Fuse declined the plate and the boss")
    solid = joined.Shape()
    solid = rounded(solid, 1.5, edges_where(solid, on_circle((0, 0, 0), 4)), "the boss's root")
    result = offset(solid, DISTANCE)
    # OCCT's offset may give the solid back as its closed shell; the solid is the one it bounds.
    if result.ShapeType() == TopAbs_ShapeEnum.TopAbs_SHELL:
        result = BRepBuilderAPI_MakeSolid(TopoDS.Shell(result)).Solid()
    return result
