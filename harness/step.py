"""Read a STEP file (ISO 10303-21) for what its solid is made of: how its boundary is represented,
its topology, and its faces and edges by the geometry under them. Standard library only.

    python3 harness/step.py FILE.step     # prints the make-up as JSON

A STEP file is a list of numbered entities, `#12=ADVANCED_FACE('',(#13),#14,.T.);`, each naming
the others it is made of by number. `read` follows those names from each face to its surface and
its edges, and from each edge to its curve; it does not check the geometry, only counts it.

The make-up (`makeup`):

- `schema`: the file's schema, as FILE_SCHEMA names it (AP203, AP214 and AP242 by their own names);
- `form`: how the solid is represented: `advanced` (a MANIFOLD_SOLID_BREP or BREP_WITH_VOIDS of
  ADVANCED_FACEs, each face on a surface of its own), `faceted` (a FACETED_BREP: polygons, no
  surfaces), `tessellated` (AP242's triangulated faces), `surfaces` (faces on surfaces, but no
  solid), or `none`;
- `solids`, `shells`, `faces`, `loops`, `edges`, `vertices`: the topology, each entity once;
- `surfaces`: the faces counted by the kind of their surface (`plane`, `cylinder`, `cone`,
  `sphere`, `torus`, `bspline`, `rational bspline`, `revolution`, `extrusion`, `offset`, or the
  entity's own name in lower case for any other);
- `curves`: the edges counted by the kind of their curve likewise (`line`, `circle`, `ellipse`,
  `bspline`, `rational bspline`, ...), a curve on surfaces (SURFACE_CURVE, SEAM_CURVE) counted
  as the curve in space it lies along;
- `triangles`: how many of the faces are flat triangles (a plane bounded by one loop of three
  edges): a mesh stored as faces;
- `tessellated`: for a tessellated file, its triangles.
"""
import json
import re
import sys

# the kinds of surface and curve, by entity name
SURFACES = {
    "PLANE": "plane",
    "CYLINDRICAL_SURFACE": "cylinder",
    "CONICAL_SURFACE": "cone",
    "SPHERICAL_SURFACE": "sphere",
    "TOROIDAL_SURFACE": "torus",
    "DEGENERATE_TOROIDAL_SURFACE": "torus",
    "B_SPLINE_SURFACE_WITH_KNOTS": "bspline",
    "B_SPLINE_SURFACE": "bspline",
    "BEZIER_SURFACE": "bspline",
    "UNIFORM_SURFACE": "bspline",
    "QUASI_UNIFORM_SURFACE": "bspline",
    "SURFACE_OF_REVOLUTION": "revolution",
    "SURFACE_OF_LINEAR_EXTRUSION": "extrusion",
    "OFFSET_SURFACE": "offset",
    "RECTANGULAR_TRIMMED_SURFACE": "trimmed",
}
CURVES = {
    "LINE": "line",
    "CIRCLE": "circle",
    "ELLIPSE": "ellipse",
    "PARABOLA": "parabola",
    "HYPERBOLA": "hyperbola",
    "B_SPLINE_CURVE_WITH_KNOTS": "bspline",
    "B_SPLINE_CURVE": "bspline",
    "BEZIER_CURVE": "bspline",
    "UNIFORM_CURVE": "bspline",
    "QUASI_UNIFORM_CURVE": "bspline",
    "POLYLINE": "polyline",
    "TRIMMED_CURVE": "trimmed",
    "OFFSET_CURVE_3D": "offset",
}
# a curve on surfaces, which names the curve in space it lies along first: counted as that curve
WRAPPERS = ("SURFACE_CURVE", "SEAM_CURVE", "INTERSECTION_CURVE")
FACES = ("ADVANCED_FACE", "FACE_SURFACE")
SOLIDS = ("MANIFOLD_SOLID_BREP", "BREP_WITH_VOIDS", "FACETED_BREP")
SHELLS = ("CLOSED_SHELL", "OPEN_SHELL", "ORIENTED_CLOSED_SHELL")
BOUNDS = ("FACE_OUTER_BOUND", "FACE_BOUND")
TESSELLATED = ("TRIANGULATED_FACE", "COMPLEX_TRIANGULATED_FACE", "TRIANGULATED_SURFACE_SET",
               "COMPLEX_TRIANGULATED_SURFACE_SET")

# a record's pieces: a string (its quote doubled within), a comment, the end of a record, or text
TOKEN = re.compile(r"'(?:[^']|'')*'|/\*.*?\*/|;|[^';/]+|/", re.S)
ENTITY = re.compile(r"\s*#(\d+)\s*=\s*(.*)", re.S)
NAME = re.compile(r"[A-Z_][A-Z0-9_]*")
REF = re.compile(r"#(\d+)")


class Entity:
    """One entity: its `names` (one, or several for a complex entity, `(A() B())`), and the
    entities it names, in order (`refs`)."""
    __slots__ = ("names", "refs", "text")

    def __init__(self, names, refs, text):
        self.names, self.refs, self.text = names, refs, text

    @property
    def name(self):
        return self.names[0]


def records(text):
    """The file's records, strings blanked to '' and comments dropped, each with its strings kept
    apart (the header's are wanted)."""
    parts, strings = [], []
    for m in TOKEN.finditer(text):
        t = m.group(0)
        if t == ";":
            yield "".join(parts), strings
            parts, strings = [], []
        elif t.startswith("'"):
            parts.append("''")
            strings.append(t[1:-1].replace("''", "'"))
        elif not t.startswith("/*"):
            parts.append(t)


def entity(body):
    """An entity's names and references, from its record's text after `#n=`."""
    body = body.strip()
    if body.startswith("("):
        # a complex entity: its parts' names, at the outer level
        names, depth, i = [], 0, 1
        while i < len(body):
            c = body[i]
            if c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
            elif depth == 0:
                m = NAME.match(body, i)
                if m:
                    names.append(m.group(0))
                    i = m.end()
                    continue
            i += 1
    else:
        m = NAME.match(body)
        names = [m.group(0)] if m else ["?"]
    return names, [int(r) for r in REF.findall(body)]


def parse(text):
    """The file's schema and its entities by number."""
    schema = None
    entities = {}
    data = False
    for body, strings in records(text):
        b = body.strip()
        if not data:
            if b.startswith("FILE_SCHEMA") and strings and strings[0].split():
                # its name, without the object identifier some writers give after it
                schema = strings[0].split()[0]
            elif b == "DATA" or b.startswith("DATA("):
                data = True
            continue
        if b == "ENDSEC":
            data = False
            continue
        m = ENTITY.match(body)
        if m:
            names, refs = entity(m.group(2))
            entities[int(m.group(1))] = Entity(names, refs, m.group(2))
    return schema, entities


def kind(e, kinds):
    """A surface's or curve's kind, by `kinds`: a rational B-spline told apart; any other entity
    by its own name in lower case."""
    rational = "RATIONAL_B_SPLINE_SURFACE" in e.names or "RATIONAL_B_SPLINE_CURVE" in e.names
    for n in e.names:
        if n in kinds:
            k = kinds[n]
            return f"rational {k}" if rational and k == "bspline" else k
    return e.names[0].lower()


def makeup(schema, entities):
    """What the solid is made of (the module's docstring says each field)."""
    by_name = {}
    for n, e in entities.items():
        for name in e.names:
            by_name.setdefault(name, []).append(n)

    def all_of(names):
        return [n for name in names for n in by_name.get(name, [])]

    def tally(counts, key):
        counts[key] = counts.get(key, 0) + 1

    solids = all_of(SOLIDS)
    faces = all_of(FACES)
    surfaces, curves = {}, {}
    loops, edges, vertices = set(), set(), set()
    triangles = 0
    for f in faces:
        face = entities[f]
        # ADVANCED_FACE('', (bounds), surface, same_sense): the surface is named last
        geometry = entities.get(face.refs[-1]) if face.refs else None
        face_kind = kind(geometry, SURFACES) if geometry else "none"
        tally(surfaces, face_kind)
        face_edges = 0
        face_loops = 0
        for b in face.refs[:-1]:
            bound = entities.get(b)
            if not bound or bound.name not in BOUNDS or not bound.refs:
                continue
            loop = entities.get(bound.refs[0])
            if not loop:
                continue
            loops.add(bound.refs[0])
            face_loops += 1
            if loop.name == "VERTEX_LOOP":
                vertices.update(loop.refs)
                continue
            for oe in loop.refs:
                oriented = entities.get(oe)
                if not oriented or not oriented.refs:
                    continue
                # ORIENTED_EDGE('', *, *, edge, sense): the edge is named last
                edges.add(oriented.refs[-1])
                face_edges += 1
        if face_kind == "plane" and face_loops == 1 and face_edges == 3:
            triangles += 1
    for e in edges:
        edge = entities.get(e)
        if not edge or edge.name != "EDGE_CURVE" or len(edge.refs) < 3:
            continue
        # EDGE_CURVE('', start, end, curve, same_sense)
        vertices.update(edge.refs[:2])
        curve = entities.get(edge.refs[2])
        for _ in range(4):
            if not curve or curve.name not in WRAPPERS or not curve.refs:
                break
            curve = entities.get(curve.refs[0])
        tally(curves, kind(curve, CURVES) if curve else "none")
    shells = {r for s in solids for r in entities[s].refs
              if entities.get(r) and entities[r].name in SHELLS}
    tessellated = all_of(TESSELLATED)
    if any(name in entities[s].names for s in solids for name in ("FACETED_BREP",)):
        form = "faceted"
    elif solids and faces:
        form = "advanced"
    elif tessellated:
        form = "tessellated"
    elif faces:
        form = "surfaces"
    else:
        form = "none"
    out = {"schema": schema, "form": form, "solids": len(solids), "shells": len(shells),
           "faces": len(faces), "loops": len(loops), "edges": len(edges),
           "vertices": len(vertices),
           "surfaces": dict(sorted(surfaces.items(), key=lambda x: -x[1])),
           "curves": dict(sorted(curves.items(), key=lambda x: -x[1])),
           "triangles": triangles}
    if tessellated:
        out["tessellated"] = len(tessellated)
    return out


def read(path):
    """The make-up of the solid in the STEP file at `path`. ValueError where it is not one."""
    with open(path, encoding="latin-1") as f:
        text = f.read()
    if "ISO-10303-21" not in text[:200]:
        raise ValueError("not a STEP file (no ISO-10303-21 header)")
    schema, entities = parse(text)
    if not entities:
        raise ValueError("no entities in its DATA section")
    return makeup(schema, entities)


if __name__ == "__main__":
    print(json.dumps(read(sys.argv[1]), indent=1))
