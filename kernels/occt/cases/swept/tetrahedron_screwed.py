"""A regular tetrahedron carried along a screw. The solid a regular tetrahedron fills as it is
carried along a screw.

The tetrahedron's corners are (5, 1, 1), (5, −1, −1), (3, 1, −1) and (3, −1, 1): the regular
tetrahedron of edge 2√2 inscribed in the cube from (3, −1, −1) to (5, 1, 1).

The screw is about the z axis: the tetrahedron turns right-handed about it (from +x toward +y) and
rises 5 for every whole turn, rigidly, from a turn of 0° to one of 540°. Make the solid of
everything the tetrahedron passes through.

Its silhouette under the screw turns at its corners, so the swept solid's walls are sheets swept by
its edges, joined where the corners' helices run, and the turns of the screw stay apart (it rises 5
a turn and is 2 tall)."""
from OCP.BRepBuilderAPI import (BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakePolygon,
                                BRepBuilderAPI_MakeSolid, BRepBuilderAPI_Sewing)
from OCP.BRepLib import BRepLib
from OCP.TopAbs import TopAbs_SHELL
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from bench import Refused
from sweep import flat_contacts_screwed, found, screwed, united

CORNERS = [(5, 1, 1), (5, -1, -1), (3, 1, -1), (3, -1, 1)]
LEAD = 5  # rise per whole turn
START, STOP = 0, 540  # degrees


def tetrahedron():
    """The four triangles between the corners, sewn into a solid."""
    sewing = BRepBuilderAPI_Sewing(1e-7)
    for i in range(4):
        corners = [gp_Pnt(*c) for j, c in enumerate(CORNERS) if j != i]
        polygon = BRepBuilderAPI_MakePolygon(*corners, True)
        sewing.Add(BRepBuilderAPI_MakeFace(polygon.Wire()).Face())
    sewing.Perform()
    solid = BRepBuilderAPI_MakeSolid(TopoDS.Shell(found(sewing.SewedShape(), TopAbs_SHELL)[0]))
    if not solid.IsDone():
        raise Refused("BRepBuilderAPI_MakeSolid declined the tetrahedron's sewn faces")
    solid = solid.Solid()
    BRepLib.OrientClosedSolid_s(solid)
    return solid


def build():
    tool = tetrahedron()
    # Each face is cut along its contact with the screw, a line parallel to the axis.
    return united(screwed(tool, LEAD, START, STOP, flat_contacts_screwed(tool, LEAD)))
