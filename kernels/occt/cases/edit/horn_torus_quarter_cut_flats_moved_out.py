"""A quarter of the upper half of a horn torus, both its cut flats moved out past the pinch. A horn torus: the solid swept by the disk of radius 5 centered on (5, 0, 0) in the plane y = 0,
turned about the z axis. Its tube touches the axis at the origin, where its surface pinches to a
point. Take its upper half, z ≥ 0, and of that the quarter turn from φ = 0° to φ = 90°, where φ is
the angle about the z axis from +x toward +y. Its faces are the torus; a flat quarter disk of radius
10 in the plane z = 0, whose corner is the pinch; and two flat half disks of radius 5 above z = 0,
one in the plane y = 0 centered on (5, 0, 0), whose outward normal is −y, and one in the plane
x = 0 centered on (0, 5, 0), whose outward normal is −x. They touch at the pinch.

Then edit the solid directly: move each of the two half disks 3 outward along its outward normal,
to the planes y = −3 and x = −3. The torus runs on round the axis to meet them, and the flat in
z = 0 runs on with it, so the two moved flats meet along the line x = y = −3, in an edge that is
there only for the heights where the tube reaches that line. The pinch, at the origin, is left a
point where the torus touches the middle of the flat in z = 0.

So the result is the upper half of the horn torus less its parts with x < −3 or y < −3. It has no
closed form that is cheap to derive: its volume is ∫ over ρ of ρ·h(ρ)·L(ρ), with h(ρ) =
√(10ρ − ρ²) the height of the tube at distance ρ from the axis and L the angle about the axis at
distance ρ where both x ≥ −3 and y ≥ −3, which is 2π for ρ ≤ 3, 2π − 4·acos(3/ρ) up to ρ = 3√2
and π/2 + 2·asin(3/ρ) beyond."""
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
    quarter = BRepPrimAPI_MakeTorus(5, 5, 0, math.pi, math.pi / 2).Shape()
    first = face_near(quarter, (5, 0, 2))   # the half disk in y = 0: its normal is -y
    second = face_near(quarter, (0, 5, 2))  # the half disk in x = 0: its normal is -x
    return moved(quarter, [(first, 3), (second, 3)])
