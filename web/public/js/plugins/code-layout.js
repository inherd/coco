// MIT License
//
// Copyright (c) 2020 Korny Sietsma
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
const debug = false;

function computeCirclingPolygon(points, radius) {
  const increment = (2 * Math.PI) / points;
  const circlingPolygon = [];

  for (let a = 0, i = 0; i < points; i++, a += increment) {
    circlingPolygon.push([radius * Math.cos(a), radius * Math.sin(a)]);
  }

  return circlingPolygon;
}

function flareWeightLoc(d) {
  if (d.data === undefined) return 0;
  if (d.data.loc === undefined) return 0;
  return d.data.loc.code;
}

function pruneWeightlessNodes(hierarchy) {
  if (hierarchy.children !== undefined) {
    // eslint-disable-next-line no-param-reassign
    hierarchy.children = hierarchy.children.filter((node) => node.value > 0);
    hierarchy.children.forEach((child) => pruneWeightlessNodes(child));
  }
}

function addPaths(pathSoFar, node) {
  let path;
  if (pathSoFar === null) {
    path = ''; // not 'flare' - could use '/' or null - but this is nicer for output
  } else {
    if (pathSoFar === '') {
      path = node.name;
    } else {
      path = `${pathSoFar}/${node.name}`;
    }
  }
  const children = node.children
    ? node.children.map((n) => addPaths(path, n))
    : undefined;
  return {
    name: node.name,
    path,
    children: children,
    layout: node.layout,
    value: node.value,
    data: node.data,
  };
}

function calculate_values(node) {
  if (node.children) {
    for (const n of node.children) {
      calculate_values(n);
    }
    const tot = node.children.map((n) => n.value).reduce((a, b) => a + b, 0);
    node.value = tot;
  } else {
    node.value = flareWeightLoc(node);
  }
}

function calculateVoronoi(
  nameSoFar,
  node,
  clipPolygon,
  center,
  goodenough,
  depth
) {
  const name = nameSoFar ? `${nameSoFar}/${node.name}` : node.name;
  node.layout = {
    polygon: clipPolygon,
    center,
    algorithm: 'voronoi',
  };

  if (!node.children) {
    return;
  }
  if (debug) {
    if (depth < 3) {
      console.warn(`calculating voronoi for ${name}`);
    } else if (depth === 3) {
      console.warn(`calculating voronoi for ${name} and descendants`);
    }

    console.warn(
      `calculating voronoi for ${name} with ${node.children.length} children and a clip polygon with ${clipPolygon.length} vertices`
    );
  }

  const MAX_SIMULATION_COUNT = 200; // we re-run the whole simulation this many times if it fails
  const MAX_ITERATION_COUNT = 500; // this is how many times a particular simulation iterates
  const MIN_WEIGHT_RATIO = 0.005; // maybe this should be a parameter? Too high, we iterate a lot.  Too low, sizes are not proportional to lines of code.
  let simulationCount = 0;
  let simulationLoopEnded = false;
  let bestConvergenceRatio = 1.0;
  let bestPolygons = undefined;
  while (!simulationLoopEnded) {

    let seed = new Math.seedrandom(20);
    var simulation = d3.voronoiMapSimulation(node.children)
      .maxIterationCount(MAX_ITERATION_COUNT)
      .minWeightRatio(MIN_WEIGHT_RATIO)
      .weight((d) => d.value)
      .prng(seed)
      .clip(clipPolygon)
      .stop();

    var state = simulation.state();

    let tickCount = 0;
    let warningTime = Date.now();
    while (!state.ended) {
      tickCount += 1;
      const now = Date.now();
      if (now - warningTime > 10000) {
        // every 10 seconds
        warningTime = now;
        console.warn(
          `slow voronoi processing of ${name} with ${node.children.length} children, tick count: ${tickCount}`
        );
      }
      simulation.tick();
      state = simulation.state();
    }
    if (tickCount === MAX_ITERATION_COUNT) {
      if (state.convergenceRatio < bestConvergenceRatio) {
        if (debug) {
          console.warn(
            'best iteration result so far',
            simulationCount,
            state.convergenceRatio
          );
        }
        bestConvergenceRatio = state.convergenceRatio;
        bestPolygons = [...state.polygons];
      }

      if (simulationCount < MAX_SIMULATION_COUNT) {
        simulationCount = simulationCount + 1;

        console.warn(
          `processing ${name} with ${node.children.length} children - Exceeded tick count ${tickCount} - retrying from scratch, try ${simulationCount}`
        );
      } else {
        console.error('Too many meta retries - stopping');
        simulationLoopEnded = true;
        if (!goodenough) {
          throw Error("Too many retries, can't provide good simulation");
        } else {
          console.warn('returning good-enough result', bestConvergenceRatio);
        }
      }
    } else {
      if (bestPolygons) {
        console.warn(
          'successful converging layout, using real ratio not best-so-far: ',
          state.convergenceRatio
        );
        bestPolygons = undefined;
        bestConvergenceRatio = state.convergenceRatio;
      }
      simulationLoopEnded = true;
    }
  }
  var polygons = state.polygons;
  if (bestPolygons) {
    console.error(
      'No good layout found - using best convergence ratio',
      bestConvergenceRatio
    );
    polygons = bestPolygons;
  } else {
    if (debug) {
      console.warn(
        'Successful layout - best convergence ratio',
        state.convergenceRatio
      );
    }
  }

  for (const polygon of polygons) {
    const pdata = polygon.map((d) => d);
    calculateVoronoi(
      name,
      polygon.site.originalObject.data.originalData,
      pdata,
      [polygon.site.x, polygon.site.y],
      goodenough,
      depth + 1
    );
  }
}

function calculateCodeLayout(input) {
  return codeLayout(input, 128, false);
}

function codeLayout(input, points, circles) {
  const parsedData = input
  const width = 1024;

  // console.warn('getting values recursively');
  calculate_values(parsedData);
  // console.warn('pruning empty nodes');
  pruneWeightlessNodes(parsedData);

  // top level clip shape
  if (circles) {
    // area = pi r^2 so r = sqrt(area/pi) or just use sqrt(area) for simplicity
    const children = parsedData.children.map((child) => {
      return {r: Math.sqrt(child.value), originalObject: child};
    });
    d3.packSiblings(children);
    // top level layout
    const enclosingCircle = d3.packEnclose(children);
    const {x, y, r} = enclosingCircle;
    // TODO: offset by x/y
    parsedData.layout = {
      polygon: computeCirclingPolygon(points, r),
      center: [0, 0],
      width: r * 2,
      height: r * 2,
      algorithm: 'circlePack',
    };

    for (const child of children) {
      const clipPolygon = computeCirclingPolygon(
        points,
        child.r
      ).map(([x, y]) => [x + child.x, y + child.y]);
      const center = [child.x, child.y];

      calculateVoronoi(
        child.originalObject.name,
        child.originalObject,
        clipPolygon,
        center,
        true,
        1
      );
      child.originalObject.layout.width = child.r;
      child.originalObject.layout.height = child.r;
    }
  } else {
    const clipPolygon = computeCirclingPolygon(points, width / 2);
    const center = [0, 0];

    calculateVoronoi(null, parsedData, clipPolygon, center, true, 0);

    parsedData.layout.width = width;
    parsedData.layout.height = width;
  }

  const results = addPaths(null, parsedData);

  return results;
}
