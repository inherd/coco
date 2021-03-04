(function (global, factory) {
  typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports, require('d3-array'), require('d3-polygon')) :
  typeof define === 'function' && define.amd ? define(['exports', 'd3-array', 'd3-polygon'], factory) :
  (factory((global.d3 = global.d3 || {}),global.d3,global.d3));
}(this, function (exports,d3Array,d3Polygon) { 'use strict';

  var epsilon = 1e-10;

  function epsilonesque(n) {
    return n <= epsilon && n >= -epsilon;
  }

  // IN: vectors or vertices
  // OUT: dot product
  function dot(v0, v1) {
    return v0.x * v1.x + v0.y * v1.y + v0.z * v1.z;
  }

  // IN: two vertex objects, v0 and v1
  // OUT: true if they are linearly dependent, false otherwise
  // from https://math.stackexchange.com/questions/1144357/how-can-i-prove-that-two-vectors-in-%E2%84%9D3-are-linearly-independent-iff-their-cro
  function linearDependent(v0, v1) {
    return (
      epsilonesque(v0.x * v1.y - v0.y * v1.x) &&
      epsilonesque(v0.y * v1.z - v0.z * v1.y) &&
      epsilonesque(v0.z * v1.x - v0.x * v1.z)
    );
  }

  // IN: an array of 2D-points [x,y]
  // OUT: true if the set defines a convex polygon (non-intersecting, hole-free, non-concave)
  // from https://gist.github.com/annatomka/82715127b74473859054, adapted to [x,y] syntax (instead of {x: ..., y: ...}) and optimizations
  function polygonDirection(polygon) {
    var direction, sign, crossproduct, p0, p1, p2, v0, v1, i;

    //begin: initialization
    p0 = polygon[polygon.length - 2];
    p1 = polygon[polygon.length - 1];
    p2 = polygon[0];
    v0 = vect(p0, p1);
    v1 = vect(p1, p2);
    crossproduct = calculateCrossproduct(v0, v1);
    // console.log(`[[${p0}], [${p1}], [${p2}]] => (${v0}) x (${v1}) = ${crossproduct}`);
    sign = Math.sign(crossproduct);
    //end: initialization

    p0 = p1; // p0 = polygon[polygon.length - 1];
    p1 = p2; // p1 = polygon[0];
    p2 = polygon[1];
    v0 = v1;
    v1 = vect(p1, p2);
    crossproduct = calculateCrossproduct(v0, v1);
    // console.log(`[[${p0}], [${p1}], [${p2}]] => (${v0}) x (${v1}) = ${crossproduct}`);
    if (Math.sign(crossproduct) !== sign) {
      return undefined;
    } //different signs in cross products means concave polygon

    //iterate on remaining 3 consecutive points
    for (i = 2; i < polygon.length - 1; i++) {
      p0 = p1;
      p1 = p2;
      p2 = polygon[i];
      v0 = v1;
      v1 = vect(p1, p2);
      crossproduct = calculateCrossproduct(v0, v1);
      // console.log(`[[${p0}], [${p1}], [${p2}]] => (${v0}) x (${v1}) = ${crossproduct}`);
      if (Math.sign(crossproduct) !== sign) {
        return undefined;
      } //different signs in cross products means concave polygon
    }

    return sign;
  }

  function vect(from, to) {
    return [to[0] - from[0], to[1] - from[1]];
  }

  function calculateCrossproduct(v0, v1) {
    return v0[0] * v1[1] - v0[1] * v1[0];
  }

  // ConflictList and ConflictListNode

  function ConflictListNode (face, vert) {
    this.face = face;
    this.vert = vert;
    this.nextf = null;
    this.prevf = null;
    this.nextv = null;
    this.prevv = null;
  }

  // IN: boolean forFace
  function ConflictList (forFace) {
    this.forFace = forFace;
    this.head = null;
  }

  // IN: ConflictListNode cln
  ConflictList.prototype.add = function(cln) {
    if (this.head === null) {
      this.head = cln;
    } else {
      if (this.forFace) {  // Is FaceList
        this.head.prevv = cln;
        cln.nextv = this.head;
        this.head = cln;
      } else {  // Is VertexList
        this.head.prevf = cln;
        cln.nextf = this.head;
        this.head = cln;
      }
    }
  }

  ConflictList.prototype.isEmpty = function() {
    return this.head === null;
  }

  // Array of faces visible
  ConflictList.prototype.fill = function(visible) {
    if (this.forFace) {
      return;
    }
    var curr = this.head;
    do {
      visible.push(curr.face);
      curr.face.marked = true;
      curr = curr.nextf;
    } while (curr !== null);
  }

  ConflictList.prototype.removeAll = function() {
    if (this.forFace) {  // Remove all vertices from Face
      var curr = this.head;
      do {
        if (curr.prevf === null) {  // Node is head
          if (curr.nextf === null) {
            curr.vert.conflicts.head = null;
          } else {
            curr.nextf.prevf = null;
            curr.vert.conflicts.head = curr.nextf;
          }
        } else {  // Node is not head
          if (curr.nextf != null) {
            curr.nextf.prevf = curr.prevf;
          }
          curr.prevf.nextf = curr.nextf;
        }
        curr = curr.nextv;
        if (curr != null) {
          curr.prevv = null;
        }
      } while (curr != null);
    } else {  // Remove all JFaces from vertex
      var curr = this.head;
      do {
        if (curr.prevv == null) {  // Node is head
          if (curr.nextv == null) {
            curr.face.conflicts.head = null;
          } else {
            curr.nextv.prevv = null;
            curr.face.conflicts.head = curr.nextv;
          }
        } else {  // Node is not head
          if (curr.nextv != null) {
            curr.nextv.prevv = curr.prevv;
          }
          curr.prevv.nextv = curr.nextv;
        }
        curr = curr.nextf;
        if (curr != null)
          curr.prevf = null;
      } while (curr != null);
    }
  }

  // IN: list of vertices
  ConflictList.prototype.getVertices = function() {
    var list = [],
    		curr = this.head;
    while (curr !== null) {
      list.push(curr.vert);
      curr = curr.nextv;
    }
    return list;
  }

  // IN: coordinates x, y, z
  function Vertex (x, y, z, weight, orig, isDummy) {
    this.x = x;
    this.y = y;
    this.weight = epsilon;
    this.index = 0;
    this.conflicts = new ConflictList(false);
    this.neighbours = null;  // Potential trouble
    this.nonClippedPolygon = null;
    this.polygon = null;
    this.originalObject = null;
    this.isDummy = false;

    if (orig !== undefined) {
      this.originalObject = orig;
    }
    if (isDummy != undefined) {
      this.isDummy = isDummy;
    }
    if (weight != null) {
      this.weight = weight;
    }
    if (z != null) {
      this.z = z;
    } else {
      this.z = this.projectZ(this.x, this.y, this.weight);
    }
  }

  Vertex.prototype.projectZ = function(x, y, weight) {
    return ((x*x) + (y*y) - weight);
  }

  Vertex.prototype.setWeight = function(weight) {
    this.weight = weight;
    this.z = this.projectZ(this.x, this.y, this.weight);
  }

  Vertex.prototype.subtract = function(v) {
    return new Vertex(v.x - this.x, v.y - this.y, v.z - this.z);
  }

  Vertex.prototype.crossproduct = function(v) {
    return new Vertex((this.y * v.z) - (this.z * v.y), (this.z * v.x) - (this.x * v.z), (this.x * v.y) - (this.y * v.x));
  }

  Vertex.prototype.equals = function(v) {
    return (this.x === v.x && this.y === v.y && this.z === v.z);
  }

  // IN: coordinates x, y, z
  function Vertex$1 (x, y, z, weight, orig, isDummy) {
    this.x = x;
    this.y = y;
    this.weight = epsilon;
    this.index = 0;
    this.conflicts = new ConflictList(false);
    this.neighbours = null;  // Potential trouble
    this.nonClippedPolygon = null;
    this.polygon = null;
    this.originalObject = null;
    this.isDummy = false;

    if (orig !== undefined) {
      this.originalObject = orig;
    }
    if (isDummy != undefined) {
      this.isDummy = isDummy;
    }
    if (weight != null) {
      this.weight = weight;
    }
    if (z != null) {
      this.z = z;
    } else {
      this.z = this.projectZ(this.x, this.y, this.weight);
    }
  }

  Vertex$1.prototype.projectZ = function(x, y, weight) {
    return ((x*x) + (y*y) - weight);
  }

  Vertex$1.prototype.setWeight = function(weight) {
    this.weight = weight;
    this.z = this.projectZ(this.x, this.y, this.weight);
  }

  Vertex$1.prototype.subtract = function(v) {
    return new Vertex$1(v.x - this.x, v.y - this.y, v.z - this.z);
  }

  Vertex$1.prototype.crossproduct = function(v) {
    return new Vertex$1((this.y * v.z) - (this.z * v.y), (this.z * v.x) - (this.x * v.z), (this.x * v.y) - (this.y * v.x));
  }

  Vertex$1.prototype.equals = function(v) {
    return (this.x === v.x && this.y === v.y && this.z === v.z);
  }

  // Plane3D and Point2D

  // IN: Face face
  function Plane3D (face) {
    var p1 = face.verts[0];
    var p2 = face.verts[1];
    var p3 = face.verts[2];
    this.a = p1.y * (p2.z-p3.z) + p2.y * (p3.z-p1.z) + p3.y * (p1.z-p2.z);
    this.b = p1.z * (p2.x-p3.x) + p2.z * (p3.x-p1.x) + p3.z * (p1.x-p2.x);
    this.c = p1.x * (p2.y-p3.y) + p2.x * (p3.y-p1.y) + p3.x * (p1.y-p2.y);
    this.d = -1 * (p1.x * (p2.y*p3.z - p3.y*p2.z) + p2.x * (p3.y*p1.z - p1.y*p3.z) + p3.x * (p1.y*p2.z - p2.y*p1.z));	
  }

  Plane3D.prototype.getNormZPlane = function() {
    return [
      -1 * (this.a / this.c),
      -1 * (this.b / this.c),
      -1 * (this.d / this.c)
    ];
  }

  // OUT: point2D
  Plane3D.prototype.getDualPointMappedToPlane = function() {
    var nplane = this.getNormZPlane();
    var dualPoint = new Point2D(nplane[0]/2, nplane[1]/2);
    return dualPoint;
  }

  // IN: doubles x and y
  function Point2D (x, y) {
    this.x = x;
    this.y = y;
  }

  // Vector

  // IN: coordinates x, y, z
  function Vector (x, y, z) {
    this.x = x;
    this.y = y;
    this.z = z;
  }

  Vector.prototype.negate = function() {
    this.x *= -1;
    this.y *= -1;
    this.z *= -1;
  }

  // Normalizes X Y and Z in-place
  Vector.prototype.normalize = function() {
    var lenght = Math.sqrt((this.x * this.x) + (this.y * this.y) + (this.z * this.z));
    if (lenght > 0) {
      this.x /= lenght;
      this.y /= lenght;
      this.z /= lenght;
    }
  }

  // HEdge

  // IN: vertex orig, vertex dest, Face face
  function HEdge (orig, dest, face) {
    this.next = null;
    this.prev = null;
    this.twin = null;
    this.orig = orig;
    this.dest = dest;
    this.iFace = face;
  }

  HEdge.prototype.isHorizon = function() {
    return this.twin !== null && !this.iFace.marked && this.twin.iFace.marked;
  }

  // IN: array horizon
  HEdge.prototype.findHorizon = function(horizon) {
    if (this.isHorizon()) {
      if (horizon.length > 0 && this === horizon[0]) {
        return;
      } else {
        horizon.push(this);
        this.next.findHorizon(horizon);
      }
    } else {
      if (this.twin !== null) {
        this.twin.next.findHorizon(horizon);
      }
    }
  }

  // IN: vertices origin and dest
  HEdge.prototype.isEqual = function(origin, dest) {
    return ((this.orig.equals(origin) && this.dest.equals(dest)) || (this.orig.equals(dest) && this.dest.equals(origin)));
  }

  // from https://stackoverflow.com/questions/1382107/whats-a-good-way-to-extend-error-in-javascript
  // (above link provided by https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error)

  function d3WeightedVoronoiError(message) {
    this.message = message;
    this.stack = new Error().stack;
  }

  d3WeightedVoronoiError.prototype.name = 'd3WeightedVoronoiError';
  d3WeightedVoronoiError.prototype = new Error();

  // IN: Vertices a, b, c
  function Face(a, b, c, orient) {
    this.conflicts = new ConflictList(true);
    this.verts = [a, b, c];
    this.marked = false;
    var t = a.subtract(b).crossproduct(b.subtract(c));

    this.normal = new Vector(-t.x, -t.y, -t.z);
    this.normal.normalize();
    this.createEdges();
    this.dualPoint = null;

    if (orient != undefined) {
      this.orient(orient);
    }
  }

  // OUT: Point2D
  Face.prototype.getDualPoint = function () {
    if (this.dualPoint == null) {
      var plane3d = new Plane3D(this);
      this.dualPoint = plane3d.getDualPointMappedToPlane();
    }
    return this.dualPoint;
  };

  Face.prototype.isVisibleFromBelow = function () {
    return this.normal.z < -1.4259414393190911e-9;
  };

  Face.prototype.createEdges = function () {
    this.edges = [];
    this.edges[0] = new HEdge(this.verts[0], this.verts[1], this);
    this.edges[1] = new HEdge(this.verts[1], this.verts[2], this);
    this.edges[2] = new HEdge(this.verts[2], this.verts[0], this);
    this.edges[0].next = this.edges[1];
    this.edges[0].prev = this.edges[2];
    this.edges[1].next = this.edges[2];
    this.edges[1].prev = this.edges[0];
    this.edges[2].next = this.edges[0];
    this.edges[2].prev = this.edges[1];
  };

  // IN: vertex orient
  Face.prototype.orient = function (orient) {
    if (!(dot(this.normal, orient) < dot(this.normal, this.verts[0]))) {
      var temp = this.verts[1];
      this.verts[1] = this.verts[2];
      this.verts[2] = temp;
      this.normal.negate();
      this.createEdges();
    }
  };

  // IN: two vertices v0 and v1
  Face.prototype.getEdge = function (v0, v1) {
    for (var i = 0; i < 3; i++) {
      if (this.edges[i].isEqual(v0, v1)) {
        return this.edges[i];
      }
    }
    return null;
  };

  // IN: Face face, vertices v0 and v1
  Face.prototype.link = function (face, v0, v1) {
    if (face instanceof Face) {
      var twin = face.getEdge(v0, v1);
      if (twin === null) {
        throw new d3WeightedVoronoiError('when linking, twin is null');
      }
      var edge = this.getEdge(v0, v1);
      if (edge === null) {
        throw new d3WeightedVoronoiError('when linking, twin is null');
      }
      twin.twin = edge;
      edge.twin = twin;
    } else {
      var twin = face; // face is a hEdge
      var edge = this.getEdge(twin.orig, twin.dest);
      twin.twin = edge;
      edge.twin = twin;
    }
  };

  // IN: vertex v
  Face.prototype.conflict = function (v) {
    return dot(this.normal, v) > dot(this.normal, this.verts[0]) + epsilon;
  };

  Face.prototype.getHorizon = function () {
    for (var i = 0; i < 3; i++) {
      if (this.edges[i].twin !== null && this.edges[i].twin.isHorizon()) {
        return this.edges[i];
      }
    }
    return null;
  };

  Face.prototype.removeConflict = function () {
    this.conflicts.removeAll();
  };

  function ConvexHull() {
    this.points = [];
    this.facets = [];
    this.created = [];
    this.horizon = [];
    this.visible = [];
    this.current = 0;
  }

  // IN: sites (x,y,z)
  ConvexHull.prototype.init = function (boundingSites, sites) {
    this.points = [];
    for (var i = 0; i < sites.length; i++) {
      this.points[i] = new Vertex$1(sites[i].x, sites[i].y, sites[i].z, null, sites[i], false);
    }
    this.points = this.points.concat(boundingSites);
  };

  ConvexHull.prototype.permutate = function () {
    var pointSize = this.points.length;
    for (var i = pointSize - 1; i > 0; i--) {
      var ra = Math.floor(Math.random() * i);
      var temp = this.points[ra];
      temp.index = i;
      var currentItem = this.points[i];
      currentItem.index = ra;
      this.points.splice(ra, 1, currentItem);
      this.points.splice(i, 1, temp);
    }
  };

  (ConvexHull.prototype.prep = function () {
    if (this.points.length <= 3) {
      throw new d3WeightedVoronoiError('Less than 4 points');
    }
    for (var i = 0; i < this.points.length; i++) {
      this.points[i].index = i;
    }

    var v0, v1, v2, v3;
    var f1, f2, f3, f0;
    v0 = this.points[0];
    v1 = this.points[1];
    v2 = v3 = null;

    for (var i = 2; i < this.points.length; i++) {
      if (!(linearDependent(v0, this.points[i]) && linearDependent(v1, this.points[i]))) {
        v2 = this.points[i];
        v2.index = 2;
        this.points[2].index = i;
        this.points.splice(i, 1, this.points[2]);
        this.points.splice(2, 1, v2);
        break;
      }
    }
    if (v2 === null) {
      throw new d3WeightedVoronoiError('v2 is null');
    }

    f0 = new Face(v0, v1, v2);
    for (var i = 3; i < this.points.length; i++) {
      if (dot(f0.normal, f0.verts[0]) !== dot(f0.normal, this.points[i])) {
        v3 = this.points[i];
        v3.index = 3;
        this.points[3].index = i;
        this.points.splice(i, 1, this.points[3]);
        this.points.splice(3, 1, v3);
        break;
      }
    }
    if (v3 === null) {
      throw new d3WeightedVoronoiError('v3 is null');
    }

    f0.orient(v3);
    f1 = new Face(v0, v2, v3, v1);
    f2 = new Face(v0, v1, v3, v2);
    f3 = new Face(v1, v2, v3, v0);
    this.addFacet(f0);
    this.addFacet(f1);
    this.addFacet(f2);
    this.addFacet(f3);
    // Connect facets
    f0.link(f1, v0, v2);
    f0.link(f2, v0, v1);
    f0.link(f3, v1, v2);
    f1.link(f2, v0, v3);
    f1.link(f3, v2, v3);
    f2.link(f3, v3, v1);
    this.current = 4;

    var v;
    for (var i = this.current; i < this.points.length; i++) {
      v = this.points[i];
      if (f0.conflict(v)) {
        this.addConflict(f0, v);
      }
      if (f1.conflict(v)) {
        this.addConflict(f1, v);
      }
      if (f2.conflict(v)) {
        this.addConflict(f2, v);
      }
      if (f3.conflict(v)) {
        this.addConflict(f3, v);
      }
    }
  }),
    // IN: Faces old1 old2 and fn
    (ConvexHull.prototype.addConflicts = function (old1, old2, fn) {
      var l1 = old1.conflicts.getVertices();
      var l2 = old2.conflicts.getVertices();
      var nCL = [];
      var v1, v2;
      var i, l;
      i = l = 0;
      // Fill the possible new Conflict List
      while (i < l1.length || l < l2.length) {
        if (i < l1.length && l < l2.length) {
          v1 = l1[i];
          v2 = l2[l];
          // If the index is the same, it's the same vertex and only 1 has to be added
          if (v1.index === v2.index) {
            nCL.push(v1);
            i++;
            l++;
          } else if (v1.index > v2.index) {
            nCL.push(v1);
            i++;
          } else {
            nCL.push(v2);
            l++;
          }
        } else if (i < l1.length) {
          nCL.push(l1[i++]);
        } else {
          nCL.push(l2[l++]);
        }
      }
      // Check if the possible conflicts are real conflicts
      for (var i = nCL.length - 1; i >= 0; i--) {
        v1 = nCL[i];
        if (fn.conflict(v1)) this.addConflict(fn, v1);
      }
    });

  // IN: Face face, Vertex v
  ConvexHull.prototype.addConflict = function (face, vert) {
    var e = new ConflictListNode(face, vert);
    face.conflicts.add(e);
    vert.conflicts.add(e);
  };

  // IN: Face f
  ConvexHull.prototype.removeConflict = function (f) {
    f.removeConflict();
    var index = f.index;
    f.index = -1;
    if (index === this.facets.length - 1) {
      this.facets.splice(this.facets.length - 1, 1);
      return;
    }
    if (index >= this.facets.length || index < 0) return;
    var last = this.facets.splice(this.facets.length - 1, 1);
    last[0].index = index;
    this.facets.splice(index, 1, last[0]);
  };

  // IN: Face face
  ConvexHull.prototype.addFacet = function (face) {
    face.index = this.facets.length;
    this.facets.push(face);
  };

  ConvexHull.prototype.compute = function () {
    this.prep();
    while (this.current < this.points.length) {
      var next = this.points[this.current];
      if (next.conflicts.isEmpty()) {
        // No conflict, point in hull
        this.current++;
        continue;
      }
      this.created = []; // TODO: make sure this is okay and doesn't dangle references
      this.horizon = [];
      this.visible = [];
      // The visible faces are also marked
      next.conflicts.fill(this.visible);
      // Horizon edges are orderly added to the horizon list
      var e;
      for (var jF = 0; jF < this.visible.length; jF++) {
        e = this.visible[jF].getHorizon();
        if (e !== null) {
          e.findHorizon(this.horizon);
          break;
        }
      }
      var last = null,
        first = null;
      // Iterate over horizon edges and create new faces oriented with the marked face 3rd unused point
      for (var hEi = 0; hEi < this.horizon.length; hEi++) {
        var hE = this.horizon[hEi];
        var fn = new Face(next, hE.orig, hE.dest, hE.twin.next.dest);
        fn.conflicts = new ConflictList(true);
        // Add to facet list
        this.addFacet(fn);
        this.created.push(fn);
        // Add new conflicts
        this.addConflicts(hE.iFace, hE.twin.iFace, fn);
        // Link the new face with the horizon edge
        fn.link(hE);
        if (last !== null) fn.link(last, next, hE.orig);
        last = fn;
        if (first === null) first = fn;
      }
      // Links the first and the last created JFace
      if (first !== null && last !== null) {
        last.link(first, next, this.horizon[0].orig);
      }
      if (this.created.length != 0) {
        // update conflict graph
        for (var f = 0; f < this.visible.length; f++) {
          this.removeConflict(this.visible[f]);
        }
        this.current++;
        this.created = [];
      }
    }
    return this.facets;
  };

  ConvexHull.prototype.clear = function () {
    this.points = [];
    this.facets = [];
    this.created = [];
    this.horizon = [];
    this.visible = [];
    this.current = 0;
  };

  function polygonClip(clip, subject) {
    // Version 0.0.0. Copyright 2017 Mike Bostock.

    // Clips the specified subject polygon to the specified clip polygon;
    // requires the clip polygon to be counterclockwise and convex.
    // https://en.wikipedia.org/wiki/Sutherland–Hodgman_algorithm
    // https://observablehq.com/@d3/polygonclip

    var input,
      closed = polygonClosed(subject),
      i = -1,
      n = clip.length - polygonClosed(clip),
      j,
      m,
      a = clip[n - 1],
      b,
      c,
      d,
      intersection;

    while (++i < n) {
      input = subject.slice();
      subject.length = 0;
      b = clip[i];
      c = input[(m = input.length - closed) - 1];
      j = -1;
      while (++j < m) {
        d = input[j];
        if (polygonInside(d, a, b)) {
          if (!polygonInside(c, a, b)) {
            intersection = polygonIntersect(c, d, a, b);
            if (isFinite(intersection[0])) {
              subject.push(intersection);
            }
          }
          subject.push(d);
        } else if (polygonInside(c, a, b)) {
          intersection = polygonIntersect(c, d, a, b);
          if (isFinite(intersection[0])) {
            subject.push(intersection);
          }
        }
        c = d;
      }
      if (closed) subject.push(subject[0]);
      a = b;
    }

    return subject;
  }

  function polygonInside(p, a, b) {
    return (b[0] - a[0]) * (p[1] - a[1]) < (b[1] - a[1]) * (p[0] - a[0]);
  }

  // Intersect two infinite lines cd and ab.
  // Return Infinity if cd and ab colinear
  function polygonIntersect(c, d, a, b) {
    var x1 = c[0],
      x3 = a[0],
      x21 = d[0] - x1,
      x43 = b[0] - x3,
      y1 = c[1],
      y3 = a[1],
      y21 = d[1] - y1,
      y43 = b[1] - y3,
      ua = (x43 * (y1 - y3) - y43 * (x1 - x3)) / (y43 * x21 - x43 * y21);
    return [x1 + ua * x21, y1 + ua * y21];
  }

  // Returns true if the polygon is closed.
  function polygonClosed(coordinates) {
    var a = coordinates[0],
      b = coordinates[coordinates.length - 1];
    return !(a[0] - b[0] || a[1] - b[1]);
  }

  // IN: HEdge edge
  function getFacesOfDestVertex(edge) {
    var faces = [];
    var previous = edge;
    var first = edge.dest;
    var site = first.originalObject;
    var neighbours = [];
    do {
      previous = previous.twin.prev;
      var siteOrigin = previous.orig.originalObject;
      if (!siteOrigin.isDummy) {
        neighbours.push(siteOrigin);
      }
      var iFace = previous.iFace;
      if (iFace.isVisibleFromBelow()) {
        faces.push(iFace);
      }
    } while (previous !== edge);
    site.neighbours = neighbours;
    return faces;
  }

  // IN: Omega = convex bounding polygon
  // IN: S = unique set of sites with weights
  // OUT: Set of lines making up the voronoi power diagram
  function computePowerDiagramIntegrated (sites, boundingSites, clippingPolygon) {
    var convexHull = new ConvexHull();
    convexHull.clear();
    convexHull.init(boundingSites, sites);

    var facets = convexHull.compute(sites);
    var polygons = []; 
    var verticesVisited = [];
    var facetCount = facets.length;

    for (var i = 0; i < facetCount; i++) {
      var facet = facets[i];
      if (facet.isVisibleFromBelow()) {
        for (var e = 0; e < 3; e++) {
          // go through the edges and start to build the polygon by going through the double connected edge list
          var edge = facet.edges[e];
          var destVertex = edge.dest;
          var site = destVertex.originalObject; 

          if (!verticesVisited[destVertex.index]) {
            verticesVisited[destVertex.index] = true;
            if (site.isDummy) {
              // Check if this is one of the sites making the bounding polygon
              continue;
            }
            // faces around the vertices which correspond to the polygon corner points
            var faces = getFacesOfDestVertex(edge);
            var protopoly = [];
            var lastX = null;
            var lastY = null;
            var dx = 1;
            var dy = 1;
            for (var j = 0; j < faces.length; j++) {
              var point = faces[j].getDualPoint();
              var x1 = point.x;
              var y1 = point.y;
              if (lastX !== null) {
                dx = lastX - x1;
                dy = lastY - y1;
                if (dx < 0) {
                  dx = -dx;
                }
                if (dy < 0) {
                  dy = -dy;
                }
              }
              if (dx > epsilon || dy > epsilon) {
                protopoly.push([x1, y1]);
                lastX = x1;
                lastY = y1;
              }
            }
            
            site.nonClippedPolygon = protopoly.reverse();
            if (!site.isDummy && d3Polygon.polygonLength(site.nonClippedPolygon) > 0) {
              var clippedPoly = polygonClip(clippingPolygon, site.nonClippedPolygon);
              site.polygon = clippedPoly;
              clippedPoly.site = site;
              if (clippedPoly.length > 0) {
                polygons.push(clippedPoly);
              }
            }
          }
        }
      }
    }
    return polygons;
  }

  function weightedVoronoi() {
    /////// Inputs ///////
    var x = function (d) {
      return d.x;
    }; // accessor to the x value
    var y = function (d) {
      return d.y;
    }; // accessor to the y value
    var weight = function (d) {
      return d.weight;
    }; // accessor to the weight
    var clip = [
      [0, 0],
      [0, 1],
      [1, 1],
      [1, 0],
    ]; // clipping polygon
    var extent = [
      [0, 0],
      [1, 1],
    ]; // extent of the clipping polygon
    var size = [1, 1]; // [width, height] of the clipping polygon

    ///////////////////////
    ///////// API /////////
    ///////////////////////

    function _weightedVoronoi(data) {
      var formatedSites;

      //begin: map sites to the expected format of PowerDiagram
      formatedSites = data.map(function (d) {
        return new Vertex(x(d), y(d), null, weight(d), d, false);
      });
      //end: map sites to the expected format of PowerDiagram

      return computePowerDiagramIntegrated(formatedSites, boundingSites(), clip);
    }

    _weightedVoronoi.x = function (_) {
      if (!arguments.length) {
        return x;
      }

      x = _;
      return _weightedVoronoi;
    };

    _weightedVoronoi.y = function (_) {
      if (!arguments.length) {
        return y;
      }

      y = _;
      return _weightedVoronoi;
    };

    _weightedVoronoi.weight = function (_) {
      if (!arguments.length) {
        return weight;
      }

      weight = _;
      return _weightedVoronoi;
    };

    _weightedVoronoi.clip = function (_) {
      var direction, xExtent, yExtent;

      if (!arguments.length) {
        return clip;
      }

      xExtent = d3Array.extent(
        _.map(function (c) {
          return c[0];
        })
      );
      yExtent = d3Array.extent(
        _.map(function (c) {
          return c[1];
        })
      );
      direction = polygonDirection(_);
      if (direction === undefined) {
        clip = d3Polygon.polygonHull(_); // ensure clip to be a convex, hole-free, counterclockwise polygon
      } else if (direction === 1) {
        clip = _.reverse(); // already convex, order array in the same direction as d3-polygon.polygonHull(...)
      } else {
        clip = _;
      }
      extent = [
        [xExtent[0], yExtent[0]],
        [xExtent[1], yExtent[1]],
      ];
      size = [xExtent[1] - xExtent[0], yExtent[1] - yExtent[0]];
      return _weightedVoronoi;
    };

    _weightedVoronoi.extent = function (_) {
      if (!arguments.length) {
        return extent;
      }

      clip = [_[0], [_[0][0], _[1][1]], _[1], [_[1][0], _[0][1]]];
      extent = _;
      size = [_[1][0] - _[0][0], _[1][1] - _[0][1]];
      return _weightedVoronoi;
    };

    _weightedVoronoi.size = function (_) {
      if (!arguments.length) {
        return size;
      }

      clip = [
        [0, 0],
        [0, _[1]],
        [_[0], _[1]],
        [_[0], 0],
      ];
      extent = [[0, 0], _];
      size = _;
      return _weightedVoronoi;
    };

    ///////////////////////
    /////// Private ///////
    ///////////////////////

    function boundingSites() {
      var minX,
        maxX,
        minY,
        maxY,
        width,
        height,
        x0,
        x1,
        y0,
        y1,
        boundingData = [],
        boundingSites = [];

      minX = extent[0][0];
      maxX = extent[1][0];
      minY = extent[0][1];
      maxY = extent[1][1];
      width = maxX - minX;
      height = maxY - minY;
      x0 = minX - width;
      x1 = maxX + width;
      y0 = minY - height;
      y1 = maxY + height;

      // MUST be counterclockwise
      // if not, may produce 'TypeError: Cannot set property 'twin' of null' during computation
      // don't know how to test as it is not exposed
      boundingData[0] = [x0, y0];
      boundingData[1] = [x0, y1];
      boundingData[2] = [x1, y1];
      boundingData[3] = [x1, y0];

      for (var i = 0; i < 4; i++) {
        boundingSites.push(
          new Vertex(
            boundingData[i][0],
            boundingData[i][1],
            null,
            epsilon,
            new Vertex(boundingData[i][0], boundingData[i][1], null, epsilon, null, true),
            true
          )
        );
      }

      return boundingSites;
    }

    return _weightedVoronoi;
  }

  exports.weightedVoronoi = weightedVoronoi;
  exports.d3WeightedVoronoiError = d3WeightedVoronoiError;

  Object.defineProperty(exports, '__esModule', { value: true });

}));