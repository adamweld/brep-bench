"""An ellipse swept along a parabola arc, its minor axis in the path's plane. An ellipse swept along an arc of a parabola, carried so that it keeps its orientation to the curve.

The path lies in the xy plane: the curve y = x²/16, from the origin to (16, 16, 0); its radius of
curvature is least at the origin, 8, and it bends toward +y.

The section is an ellipse centred on the path and square to it, with semi-axis 3 along the path's
principal normal (the direction in the xy plane toward the side the path bends to, which at the
origin is +y) and semi-axis 5 along the normal to the xy plane. This stays so all along the path:
the section is carried in the path's Frenet frame, tangent, principal normal and binormal. The
ends are flat, square to the path: in the plane x = 0, and in the plane through (16, 16, 0) square
to the path there. The wall does not meet itself (the section's reach toward the bend, 3, is under
the radius of curvature, 8).

The volume is πab·L with a = 5, b = 3 and L = 8√5 + 4·asinh 2 the path's length, as the section's
center is on the path; the wall's area has no closed form."""
from OCP.BRepBuilderAPI import BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeWire
from OCP.BRepOffsetAPI import BRepOffsetAPI_MakePipeShell
from OCP.gp import gp, gp_Ax2, gp_Circ, gp_Dir, gp_Elips, gp_Parab, gp_Pnt

from bench import Refused


def pipe(path_edge, section_edge):
    """The section swept along the path in the Frenet frame, closed with flat ends."""
    pipe = BRepOffsetAPI_MakePipeShell(BRepBuilderAPI_MakeWire(path_edge).Wire())
    pipe.SetMode(True)  # Frenet: the section keeps its orientation to the path
    pipe.Add(BRepBuilderAPI_MakeWire(section_edge).Wire())
    pipe.Build()
    if not pipe.IsDone():
        raise Refused("BRepOffsetAPI_MakePipeShell could not sweep the section")
    if not pipe.MakeSolid():
        raise Refused("the swept pipe did not close into a solid")
    return pipe.Shape()


def build():
    parabola = gp_Parab(gp_Ax2(gp_Pnt(0, 0, 0), gp_Dir(0, 0, -1), gp_Dir(0, 1, 0)), 4)
    path = BRepBuilderAPI_MakeEdge(parabola, 0, 16).Edge()
    # at the origin the path leaves along +x and bends toward +y: the principal normal is +y and
    # the binormal +z, so the 5 semi-axis lies along z and the 3 along y
    section = BRepBuilderAPI_MakeEdge(
        gp_Elips(gp_Ax2(gp_Pnt(0, 0, 0), gp.DX_s(), gp.DZ_s()), 5, 3)).Edge()
    return pipe(path, section)
