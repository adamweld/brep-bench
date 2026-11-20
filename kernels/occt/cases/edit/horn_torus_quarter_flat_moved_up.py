"""A quarter of the upper half of a horn torus, its flat moved up past the pinch. A horn torus: the solid swept by the disk of radius 5 centered on (5, 0, 0) in the plane y = 0,
turned about the z axis. Its tube touches the axis at the origin, where its surface pinches to a
point. Take its upper half, z ≥ 0, and of that the quarter turn from φ = 0° to φ = 90°, where φ is
the angle about the z axis from +x toward +y. Its faces are the torus; a flat quarter disk of radius
10 in the plane z = 0, whose corner is the pinch; and two flat half disks of radius 5 above z = 0,
one in the plane y = 0 centered on (5, 0, 0) and one in the plane x = 0 centered on (0, 5, 0),
which touch at the pinch.

Then edit the solid directly: move the quarter disk in z = 0 up 3, against its outward normal −z,
to the plane z = 3. The torus is trimmed there, so the flat becomes the quarter of the annulus
between radii 1 and 9 in the plane z = 3, and the two half disks are trimmed to the circular
segments of their disks above z = 3. The pinch is no longer on the solid.

So the result is the quarter, φ from 0° to 90°, of the horn torus above z = 3."""
import math

from OCP.BRepGProp import BRepGProp
from OCP.BRepPrimAPI import BRepPrimAPI_MakeTorus
from OCP.GProp import GProp_GProps
from OCP.TopAbs import TopAbs_FACE
from OCP.TopExp import TopExp_Explorer
from OCP.TopoDS import TopoDS
from OCP.gp import gp_Pnt

from bench import face_through
from edit import moved


def face_near(solid, point):
    """The face of `solid` whose center of mass is nearest `point`."""
    best = None
    faces = TopExp_Explorer(solid, TopAbs_FACE)
    while faces.More():
        face = TopoDS.Face(faces.Current())
        props = GProp_GProps()
        BRepGProp.SurfaceProperties_s(face, props)
        gap = props.CentreOfMass().Distance(gp_Pnt(*point))
        if best is None or gap < best[0]:
            best = gap, face
        faces.Next()
    return best[1]


def build():
    # the upper half of the tube's minor angle, a quarter turn about the axis
    quarter = BRepPrimAPI_MakeTorus(5, 5, 0, math.pi, math.pi / 2).Shape()
    flat = face_through(quarter, (3, 1, 0))  # the quarter disk in z = 0; its normal is -z
    return moved(quarter, [(flat, -3)])
