(function (global, factory) {
  typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports, require('d3-polygon'), require('d3-timer'), require('d3-dispatch'), require('d3-weighted-voronoi')) :
  typeof define === 'function' && define.amd ? define(['exports', 'd3-polygon', 'd3-timer', 'd3-dispatch', 'd3-weighted-voronoi'], factory) :
  (factory((global.d3 = global.d3 || {}),global.d3,global.d3,global.d3,global.d3));
}(this, function (exports,d3Polygon,d3Timer,d3Dispatch,d3WeightedVoronoi) { 'use strict';

  function FlickeringMitigation () {
    /////// Inputs ///////
    this.growthChangesLength = DEFAULT_LENGTH;
    this.totalAvailableArea = NaN;

    //begin: internals
    this.lastAreaError = NaN;
    this.lastGrowth = NaN;
    this.growthChanges = [];
    this.growthChangeWeights = generateGrowthChangeWeights(this.growthChangesLength); //used to make recent changes weighter than older changes
    this.growthChangeWeightsSum = computeGrowthChangeWeightsSum(this.growthChangeWeights);
    //end: internals
  }

  var DEFAULT_LENGTH = 10;

  function direction(h0, h1) {
    return (h0 >= h1)? 1 : -1;
  }

  function generateGrowthChangeWeights(length) {
    var initialWeight = 3;   // a magic number
    var weightDecrement = 1; // a magic number
    var minWeight = 1;

    var weightedCount = initialWeight;
    var growthChangeWeights = [];

    for (var i=0; i<length; i++) {
      growthChangeWeights.push(weightedCount);
      weightedCount -= weightDecrement;
      if (weightedCount<minWeight) { weightedCount = minWeight; }
    }
    return growthChangeWeights;
  }

  function computeGrowthChangeWeightsSum (growthChangeWeights) {
    var growthChangeWeightsSum = 0;
    for (var i=0; i<growthChangeWeights.length; i++) {
      growthChangeWeightsSum += growthChangeWeights[i];
    }
    return growthChangeWeightsSum;
  }

  ///////////////////////
  ///////// API /////////
  ///////////////////////

  FlickeringMitigation.prototype.reset = function () {
    this.lastAreaError = NaN;
    this.lastGrowth = NaN;
    this.growthChanges = [];
    this.growthChangesLength = DEFAULT_LENGTH;
    this.growthChangeWeights = generateGrowthChangeWeights(this.growthChangesLength);
    this.growthChangeWeightsSum = computeGrowthChangeWeightsSum(this.growthChangeWeights);
    this.totalAvailableArea = NaN;

    return this;
  };

  FlickeringMitigation.prototype.clear = function () {
    this.lastAreaError = NaN;
    this.lastGrowth = NaN;
    this.growthChanges = [];

    return this;
  };

  FlickeringMitigation.prototype.length = function (_) {
    if (!arguments.length) { return this.growthChangesLength; }

    if (parseInt(_)>0) {
      this.growthChangesLength = Math.floor(parseInt(_));
      this.growthChangeWeights = generateGrowthChangeWeights(this.growthChangesLength);
      this.growthChangeWeightsSum = computeGrowthChangeWeightsSum(this.growthChangeWeights);
    } else {
      console.warn("FlickeringMitigation.length() accepts only positive integers; unable to handle "+_);
    }
    return this;
  };

  FlickeringMitigation.prototype.totalArea = function (_) {
    if (!arguments.length) { return this.totalAvailableArea; }

    if (parseFloat(_)>0) {
      this.totalAvailableArea = parseFloat(_);
    } else {
      console.warn("FlickeringMitigation.totalArea() accepts only positive numbers; unable to handle "+_);
    }
    return this;
  };

  FlickeringMitigation.prototype.add = function (areaError) {
    var secondToLastAreaError, secondToLastGrowth;

    secondToLastAreaError = this.lastAreaError;
    this.lastAreaError = areaError;
    if (!isNaN(secondToLastAreaError)) {
      secondToLastGrowth = this.lastGrowth;
      this.lastGrowth = direction(this.lastAreaError, secondToLastAreaError);
    }
    if (!isNaN(secondToLastGrowth)) {
      this.growthChanges.unshift(this.lastGrowth!=secondToLastGrowth);
    }

    if (this.growthChanges.length>this.growthChangesLength) {
      this.growthChanges.pop();
    }
    return this;
  };

  FlickeringMitigation.prototype.ratio = function () {
    var weightedChangeCount = 0;
    var ratio;

    if (this.growthChanges.length < this.growthChangesLength) { return 0; }
    if (this.lastAreaError > this.totalAvailableArea/10) { return 0; }

    for(var i=0; i<this.growthChangesLength; i++) {
      if (this.growthChanges[i]) {
        weightedChangeCount += this.growthChangeWeights[i];
      }
    }

    ratio = weightedChangeCount/this.growthChangeWeightsSum;

    /*
    if (ratio>0) {
      console.log("flickering mitigation ratio: "+Math.floor(ratio*1000)/1000);
    }
    */

    return ratio;
  };

  function randomInitialPosition () {

    //begin: internals
    var clippingPolygon,
      extent,
      minX, maxX,
      minY, maxY,
      dx, dy;
    //end: internals

    ///////////////////////
    ///////// API /////////
    ///////////////////////

    function _random(d, i, arr, voronoiMapSimulation) {
      var shouldUpdateInternals = false;
      var x, y;

      if (clippingPolygon !== voronoiMapSimulation.clip()) {
        clippingPolygon = voronoiMapSimulation.clip();
        extent = voronoiMapSimulation.extent();
        shouldUpdateInternals = true;
      }

      if (shouldUpdateInternals) {
        updateInternals();
      }

      x = minX + dx * voronoiMapSimulation.prng()();
      y = minY + dy * voronoiMapSimulation.prng()();
      while (!d3Polygon.polygonContains(clippingPolygon, [x, y])) {
        x = minX + dx * voronoiMapSimulation.prng()();
        y = minY + dy * voronoiMapSimulation.prng()();
      }
      return [x, y];
    };

    ///////////////////////
    /////// Private ///////
    ///////////////////////

    function updateInternals() {
      minX = extent[0][0];
      maxX = extent[1][0];
      minY = extent[0][1];
      maxY = extent[1][1];
      dx = maxX - minX;
      dy = maxY - minY;
    };

    return _random;
  };

  function pie () {
    //begin: internals
    var startAngle = 0;
    var clippingPolygon,
      dataArray,
      dataArrayLength,
      clippingPolygonCentroid,
      halfIncircleRadius,
      angleBetweenData;
    //end: internals

    ///////////////////////
    ///////// API /////////
    ///////////////////////

    function _pie(d, i, arr, voronoiMapSimulation) {
      var shouldUpdateInternals = false;

      if (clippingPolygon !== voronoiMapSimulation.clip()) {
        clippingPolygon = voronoiMapSimulation.clip();
        shouldUpdateInternals |= true;
      }
      if (dataArray !== arr) {
        dataArray = arr;
        shouldUpdateInternals |= true;
      }

      if (shouldUpdateInternals) {
        updateInternals();
      }

      // add some randomness to prevent colinear/cocircular points
      // substract -0.5 so that the average jitter is still zero
      return [
        clippingPolygonCentroid[0] + Math.cos(startAngle + i * angleBetweenData) * halfIncircleRadius + (voronoiMapSimulation.prng()() - 0.5) * 1E-3,
        clippingPolygonCentroid[1] + Math.sin(startAngle + i * angleBetweenData) * halfIncircleRadius + (voronoiMapSimulation.prng()() - 0.5) * 1E-3
      ];
    };

    _pie.startAngle = function (_) {
      if (!arguments.length) {
        return startAngle;
      }

      startAngle = _;
      return _pie;
    };

    ///////////////////////
    /////// Private ///////
    ///////////////////////

    function updateInternals() {
      clippingPolygonCentroid = d3Polygon.polygonCentroid(clippingPolygon);
      halfIncircleRadius = computeMinDistFromEdges(clippingPolygonCentroid, clippingPolygon) / 2;
      dataArrayLength = dataArray.length;
      angleBetweenData = 2 * Math.PI / dataArrayLength;
    };

    function computeMinDistFromEdges(vertex, clippingPolygon) {
      var minDistFromEdges = Infinity,
        edgeIndex = 0,
        edgeVertex0 = clippingPolygon[clippingPolygon.length - 1],
        edgeVertex1 = clippingPolygon[edgeIndex];
      var distFromCurrentEdge;

      while (edgeIndex < clippingPolygon.length) {
        distFromCurrentEdge = vDistance(vertex, edgeVertex0, edgeVertex1);
        if (distFromCurrentEdge < minDistFromEdges) {
          minDistFromEdges = distFromCurrentEdge;
        }
        edgeIndex++;
        edgeVertex0 = edgeVertex1;
        edgeVertex1 = clippingPolygon[edgeIndex];
      }

      return minDistFromEdges;
    }

    //from https://stackoverflow.com/questions/849211/shortest-distance-between-a-point-and-a-line-segment
    function vDistance(vertex, edgeVertex0, edgeVertex1) {
      var x = vertex[0],
        y = vertex[1],
        x1 = edgeVertex0[0],
        y1 = edgeVertex0[1],
        x2 = edgeVertex1[0],
        y2 = edgeVertex1[1];
      var A = x - x1,
        B = y - y1,
        C = x2 - x1,
        D = y2 - y1;
      var dot = A * C + B * D;
      var len_sq = C * C + D * D;
      var param = -1;

      if (len_sq != 0) //in case of 0 length line
        param = dot / len_sq;

      var xx, yy;

      if (param < 0) { // this should not arise as clippingpolygon is convex
        xx = x1;
        yy = y1;
      } else if (param > 1) { // this should not arise as clippingpolygon is convex
        xx = x2;
        yy = y2;
      } else {
        xx = x1 + param * C;
        yy = y1 + param * D;
      }

      var dx = x - xx;
      var dy = y - yy;
      return Math.sqrt(dx * dx + dy * dy);
    }

    return _pie;
  }

  function halfAverageAreaInitialWeight () {
    //begin: internals
    var clippingPolygon,
      dataArray,
      siteCount,
      totalArea,
      halfAverageArea;
    //end: internals

    ///////////////////////
    ///////// API /////////
    ///////////////////////
    function _halfAverageArea(d, i, arr, voronoiMapSimulation) {
      var shouldUpdateInternals = false;
      if (clippingPolygon !== voronoiMapSimulation.clip()) {
        clippingPolygon = voronoiMapSimulation.clip();
        shouldUpdateInternals |= true;
      }
      if (dataArray !== arr) {
        dataArray = arr;
        shouldUpdateInternals |= true;
      }

      if (shouldUpdateInternals) {
        updateInternals();
      }

      return halfAverageArea;
    };

    ///////////////////////
    /////// Private ///////
    ///////////////////////

    function updateInternals() {
      siteCount = dataArray.length;
      totalArea = d3Polygon.polygonArea(clippingPolygon);
      halfAverageArea = totalArea / siteCount / 2; // half of the average area of the the clipping polygon
    }

    return _halfAverageArea;
  };

  // from https://stackoverflow.com/questions/1382107/whats-a-good-way-to-extend-error-in-javascript
  // (above link provided by https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Error)

  function d3VoronoiMapError(message) {
    this.message = message;
    this.stack = new Error().stack;
  }

  d3VoronoiMapError.prototype.name = 'd3VoronoiMapError';
  d3VoronoiMapError.prototype = new Error();

  function voronoiMapSimulation(data) {
    //begin: constants
    var DEFAULT_CONVERGENCE_RATIO = 0.01;
    var DEFAULT_MAX_ITERATION_COUNT = 50;
    var DEFAULT_MIN_WEIGHT_RATIO = 0.01;
    var DEFAULT_PRNG = Math.random;
    var DEFAULT_INITIAL_POSITION = randomInitialPosition();
    var DEFAULT_INITIAL_WEIGHT = halfAverageAreaInitialWeight();
    var RANDOM_INITIAL_POSITION = randomInitialPosition();
    var epsilon = 1e-10;
    //end: constants

    /////// Inputs ///////
    var weight = function (d) {
      return d.weight;
    }; // accessor to the weight
    var convergenceRatio = DEFAULT_CONVERGENCE_RATIO; // targeted allowed error ratio; default 0.01 stops computation when cell areas error <= 1% clipping polygon's area
    var maxIterationCount = DEFAULT_MAX_ITERATION_COUNT; // maximum allowed iteration; stops computation even if convergence is not reached; use a large amount for a sole converge-based computation stop
    var minWeightRatio = DEFAULT_MIN_WEIGHT_RATIO; // used to compute the minimum allowed weight; default 0.01 means 1% of max weight; handle near-zero weights, and leaves enought space for cell hovering
    var prng = DEFAULT_PRNG; // pseudorandom number generator
    var initialPosition = DEFAULT_INITIAL_POSITION; // accessor to the initial position; defaults to a random position inside the clipping polygon
    var initialWeight = DEFAULT_INITIAL_WEIGHT; // accessor to the initial weight; defaults to the average area of the clipping polygon

    //begin: internals
    var weightedVoronoi = d3WeightedVoronoi.weightedVoronoi(),
      flickeringMitigation = new FlickeringMitigation(),
      shouldInitialize = true, // should initialize due to changes via APIs
      siteCount, // number of sites
      totalArea, // area of the clipping polygon
      areaErrorTreshold, // targeted allowed area error (= totalArea * convergenceRatio); below this treshold, map is considered obtained and computation stops
      iterationCount, // current iteration
      polygons, // current computed polygons
      areaError, // current area error
      converged, // true if (areaError < areaErrorTreshold)
      ended; // stores if computation is ended, either if computation has converged or if it has reached the maximum allowed iteration
    //end: internals
    //being: internals/simulation
    var simulation,
      stepper = d3Timer.timer(step),
      event = d3Dispatch.dispatch('tick', 'end');
    //end: internals/simulation

    //begin: algorithm conf.
    const HANDLE_OVERWEIGHTED_VARIANT = 1; // this option still exists 'cause for further experiments
    const HANLDE_OVERWEIGHTED_MAX_ITERATION_COUNT = 1000; // max number of tries to handle overweigthed sites
    var handleOverweighted;
    //end: algorithm conf.

    //begin: utils
    function sqr(d) {
      return Math.pow(d, 2);
    }

    function squaredDistance(s0, s1) {
      return sqr(s1.x - s0.x) + sqr(s1.y - s0.y);
    }
    //end: utils

    ///////////////////////
    ///////// API /////////
    ///////////////////////

    simulation = {
      tick: tick,

      restart: function () {
        stepper.restart(step);
        return simulation;
      },

      stop: function () {
        stepper.stop();
        return simulation;
      },

      weight: function (_) {
        if (!arguments.length) {
          return weight;
        }

        weight = _;
        shouldInitialize = true;
        return simulation;
      },

      convergenceRatio: function (_) {
        if (!arguments.length) {
          return convergenceRatio;
        }

        convergenceRatio = _;
        shouldInitialize = true;
        return simulation;
      },

      maxIterationCount: function (_) {
        if (!arguments.length) {
          return maxIterationCount;
        }

        maxIterationCount = _;
        return simulation;
      },

      minWeightRatio: function (_) {
        if (!arguments.length) {
          return minWeightRatio;
        }

        minWeightRatio = _;
        shouldInitialize = true;
        return simulation;
      },

      clip: function (_) {
        if (!arguments.length) {
          return weightedVoronoi.clip();
        }

        weightedVoronoi.clip(_);
        shouldInitialize = true;
        return simulation;
      },

      extent: function (_) {
        if (!arguments.length) {
          return weightedVoronoi.extent();
        }

        weightedVoronoi.extent(_);
        shouldInitialize = true;
        return simulation;
      },

      size: function (_) {
        if (!arguments.length) {
          return weightedVoronoi.size();
        }

        weightedVoronoi.size(_);
        shouldInitialize = true;
        return simulation;
      },

      prng: function (_) {
        if (!arguments.length) {
          return prng;
        }

        prng = _;
        shouldInitialize = true;
        return simulation;
      },

      initialPosition: function (_) {
        if (!arguments.length) {
          return initialPosition;
        }

        initialPosition = _;
        shouldInitialize = true;
        return simulation;
      },

      initialWeight: function (_) {
        if (!arguments.length) {
          return initialWeight;
        }

        initialWeight = _;
        shouldInitialize = true;
        return simulation;
      },

      state: function () {
        if (shouldInitialize) {
          initializeSimulation();
        }
        return {
          ended: ended,
          iterationCount: iterationCount,
          convergenceRatio: areaError / totalArea,
          polygons: polygons,
        };
      },

      on: function (name, _) {
        if (arguments.length === 1) {
          return event.on(name);
        }

        event.on(name, _);
        return simulation;
      },
    };

    ///////////////////////
    /////// Private ///////
    ///////////////////////

    //begin: simulation's main loop
    function step() {
      tick();
      event.call('tick', simulation);
      if (ended) {
        stepper.stop();
        event.call('end', simulation);
      }
    }
    //end: simulation's main loop

    //begin: algorithm used at each iteration
    function tick() {
      if (!ended) {
        if (shouldInitialize) {
          initializeSimulation();
        }
        polygons = adapt(polygons, flickeringMitigation.ratio());
        iterationCount++;
        areaError = computeAreaError(polygons);
        flickeringMitigation.add(areaError);
        converged = areaError < areaErrorTreshold;
        ended = converged || iterationCount >= maxIterationCount;
        // console.log("error %: "+Math.round(areaError*100*1000/totalArea)/1000);
      }
    }
    //end: algorithm used at each iteration

    function initializeSimulation() {
      //begin: handle algorithm's variants
      setHandleOverweighted();
      //end: handle algorithm's variants

      siteCount = data.length;
      totalArea = Math.abs(d3Polygon.polygonArea(weightedVoronoi.clip()));
      areaErrorTreshold = convergenceRatio * totalArea;
      flickeringMitigation.clear().totalArea(totalArea);

      iterationCount = 0;
      converged = false;
      polygons = initialize(data, simulation);
      ended = false;
      shouldInitialize = false;
    }

    function initialize(data, simulation) {
      var maxWeight = data.reduce(function (max, d) {
          return Math.max(max, weight(d));
        }, -Infinity),
        minAllowedWeight = maxWeight * minWeightRatio;
      var weights, mapPoints;

      //begin: extract weights
      weights = data.map(function (d, i, arr) {
        return {
          index: i,
          weight: Math.max(weight(d), minAllowedWeight),
          initialPosition: initialPosition(d, i, arr, simulation),
          initialWeight: initialWeight(d, i, arr, simulation),
          originalData: d,
        };
      });
      //end: extract weights

      // create map-related points
      // (with targetedArea, initial position and initialWeight)
      mapPoints = createMapPoints(weights, simulation);
      handleOverweighted(mapPoints);
      return weightedVoronoi(mapPoints);
    }

    function createMapPoints(basePoints, simulation) {
      var totalWeight = basePoints.reduce(function (acc, bp) {
        return (acc += bp.weight);
      }, 0);
      var initialPosition;

      return basePoints.map(function (bp, i, bps) {
        initialPosition = bp.initialPosition;

        if (!d3Polygon.polygonContains(weightedVoronoi.clip(), initialPosition)) {
          initialPosition = DEFAULT_INITIAL_POSITION(bp, i, bps, simulation);
        }

        return {
          index: bp.index,
          targetedArea: (totalArea * bp.weight) / totalWeight,
          data: bp,
          x: initialPosition[0],
          y: initialPosition[1],
          weight: bp.initialWeight, // ArlindNocaj/Voronoi-Treemap-Library uses an epsilonesque initial weight; using heavier initial weights allows faster weight adjustements, hence faster stabilization
        };
      });
    }

    function adapt(polygons, flickeringMitigationRatio) {
      var adaptedMapPoints;

      adaptPositions(polygons, flickeringMitigationRatio);
      adaptedMapPoints = polygons.map(function (p) {
        return p.site.originalObject;
      });
      polygons = weightedVoronoi(adaptedMapPoints);
      if (polygons.length < siteCount) {
        throw new d3VoronoiMapError('at least 1 site has no area, which is not supposed to arise');
      }

      adaptWeights(polygons, flickeringMitigationRatio);
      adaptedMapPoints = polygons.map(function (p) {
        return p.site.originalObject;
      });
      polygons = weightedVoronoi(adaptedMapPoints);
      if (polygons.length < siteCount) {
        throw new d3VoronoiMapError('at least 1 site has no area, which is not supposed to arise');
      }

      return polygons;
    }

    function adaptPositions(polygons, flickeringMitigationRatio) {
      var newMapPoints = [],
        flickeringInfluence = 0.5;
      var flickeringMitigation, d, polygon, mapPoint, centroid, dx, dy;

      flickeringMitigation = flickeringInfluence * flickeringMitigationRatio;
      d = 1 - flickeringMitigation; // in [0.5, 1]
      for (var i = 0; i < siteCount; i++) {
        polygon = polygons[i];
        mapPoint = polygon.site.originalObject;
        centroid = d3Polygon.polygonCentroid(polygon);

        dx = centroid[0] - mapPoint.x;
        dy = centroid[1] - mapPoint.y;

        //begin: handle excessive change;
        dx *= d;
        dy *= d;
        //end: handle excessive change;

        mapPoint.x += dx;
        mapPoint.y += dy;

        newMapPoints.push(mapPoint);
      }

      handleOverweighted(newMapPoints);
    }

    function adaptWeights(polygons, flickeringMitigationRatio) {
      var newMapPoints = [],
        flickeringInfluence = 0.1;
      var flickeringMitigation, polygon, mapPoint, currentArea, adaptRatio, adaptedWeight;

      flickeringMitigation = flickeringInfluence * flickeringMitigationRatio;
      for (var i = 0; i < siteCount; i++) {
        polygon = polygons[i];
        mapPoint = polygon.site.originalObject;
        currentArea = d3Polygon.polygonArea(polygon);
        adaptRatio = mapPoint.targetedArea / currentArea;

        //begin: handle excessive change;
        adaptRatio = Math.max(adaptRatio, 1 - flickeringInfluence + flickeringMitigation); // in [(1-flickeringInfluence), 1]
        adaptRatio = Math.min(adaptRatio, 1 + flickeringInfluence - flickeringMitigation); // in [1, (1+flickeringInfluence)]
        //end: handle excessive change;

        adaptedWeight = mapPoint.weight * adaptRatio;
        adaptedWeight = Math.max(adaptedWeight, epsilon);

        mapPoint.weight = adaptedWeight;

        newMapPoints.push(mapPoint);
      }

      handleOverweighted(newMapPoints);
    }

    // heuristics: lower heavy weights
    function handleOverweighted0(mapPoints) {
      var fixCount = 0;
      var fixApplied, tpi, tpj, weightest, lightest, sqrD, adaptedWeight;
      do {
        if (fixCount > HANLDE_OVERWEIGHTED_MAX_ITERATION_COUNT) {
          throw new d3VoronoiMapError('handleOverweighted0 is looping too much');
        }
        fixApplied = false;
        for (var i = 0; i < siteCount; i++) {
          tpi = mapPoints[i];
          for (var j = i + 1; j < siteCount; j++) {
            tpj = mapPoints[j];
            if (tpi.weight > tpj.weight) {
              weightest = tpi;
              lightest = tpj;
            } else {
              weightest = tpj;
              lightest = tpi;
            }
            sqrD = squaredDistance(tpi, tpj);
            if (sqrD < weightest.weight - lightest.weight) {
              // adaptedWeight = sqrD - epsilon; // as in ArlindNocaj/Voronoi-Treemap-Library
              // adaptedWeight = sqrD + lightest.weight - epsilon; // works, but below heuristics performs better (less flickering)
              adaptedWeight = sqrD + lightest.weight / 2;
              adaptedWeight = Math.max(adaptedWeight, epsilon);
              weightest.weight = adaptedWeight;
              fixApplied = true;
              fixCount++;
              break;
            }
          }
          if (fixApplied) {
            break;
          }
        }
      } while (fixApplied);

      /*
      if (fixCount > 0) {
        console.log('# fix: ' + fixCount);
      }
      */
    }

    // heuristics: increase light weights
    function handleOverweighted1(mapPoints) {
      var fixCount = 0;
      var fixApplied, tpi, tpj, weightest, lightest, sqrD, overweight;
      do {
        if (fixCount > HANLDE_OVERWEIGHTED_MAX_ITERATION_COUNT) {
          throw new d3VoronoiMapError('handleOverweighted1 is looping too much');
        }
        fixApplied = false;
        for (var i = 0; i < siteCount; i++) {
          tpi = mapPoints[i];
          for (var j = i + 1; j < siteCount; j++) {
            tpj = mapPoints[j];
            if (tpi.weight > tpj.weight) {
              weightest = tpi;
              lightest = tpj;
            } else {
              weightest = tpj;
              lightest = tpi;
            }
            sqrD = squaredDistance(tpi, tpj);
            if (sqrD < weightest.weight - lightest.weight) {
              overweight = weightest.weight - lightest.weight - sqrD;
              lightest.weight += overweight + epsilon;
              fixApplied = true;
              fixCount++;
              break;
            }
          }
          if (fixApplied) {
            break;
          }
        }
      } while (fixApplied);

      /*
      if (fixCount > 0) {
        console.log('# fix: ' + fixCount);
      }
      */
    }

    function computeAreaError(polygons) {
      //convergence based on summation of all sites current areas
      var areaErrorSum = 0;
      var polygon, mapPoint, currentArea;
      for (var i = 0; i < siteCount; i++) {
        polygon = polygons[i];
        mapPoint = polygon.site.originalObject;
        currentArea = d3Polygon.polygonArea(polygon);
        areaErrorSum += Math.abs(mapPoint.targetedArea - currentArea);
      }
      return areaErrorSum;
    }

    function setHandleOverweighted() {
      switch (HANDLE_OVERWEIGHTED_VARIANT) {
        case 0:
          handleOverweighted = handleOverweighted0;
          break;
        case 1:
          handleOverweighted = handleOverweighted1;
          break;
        default:
          console.error("unknown 'handleOverweighted' variant; using variant #1");
          handleOverweighted = handleOverweighted0;
      }
    }

    return simulation;
  }

  exports.voronoiMapSimulation = voronoiMapSimulation;
  exports.voronoiMapInitialPositionRandom = randomInitialPosition;
  exports.voronoiMapInitialPositionPie = pie;
  exports.d3VoronoiMapError = d3VoronoiMapError;

  Object.defineProperty(exports, '__esModule', { value: true });

}));