(function (global, factory) {
  typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports, require('d3-voronoi-map')) :
  typeof define === 'function' && define.amd ? define(['exports', 'd3-voronoi-map'], factory) :
  (factory((global.d3 = global.d3 || {}),global.d3));
}(this, function (exports,voronoiMap) { 'use strict';

  voronoiMap = 'default' in voronoiMap ? voronoiMap['default'] : voronoiMap;

  function voronoiTreemap() {
    //begin: constants
    var DEFAULT_CONVERGENCE_RATIO = 0.01;
    var DEFAULT_MAX_ITERATION_COUNT = 50;
    var DEFAULT_MIN_WEIGHT_RATIO = 0.01;
    var DEFAULT_PRNG = Math.random;
    //end: constants

    /////// Inputs ///////
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
    var convergenceRatio = DEFAULT_CONVERGENCE_RATIO; // targeted allowed error ratio; default 0.01 stops computation when cell areas error <= 1% clipping polygon's area
    var maxIterationCount = DEFAULT_MAX_ITERATION_COUNT; // maximum allowed iteration; stops computation even if convergence is not reached; use a large amount for a sole converge-based computation stop
    var minWeightRatio = DEFAULT_MIN_WEIGHT_RATIO; // used to compute the minimum allowed weight; default 0.01 means 1% of max weight; handle near-zero weights, and leaves enought space for cell hovering
    var prng = DEFAULT_PRNG; // pseudorandom number generator

    //begin: internals
    var unrelevantButNeedeData = [
      {
        weight: 1,
      },
      {
        weight: 1,
      },
    ];
    var _convenientReusableVoronoiMap = voronoiMap.voronoiMapSimulation(unrelevantButNeedeData).stop();
    //end: internals

    ///////////////////////
    ///////// API /////////
    ///////////////////////

    function _voronoiTreemap(rootNode) {
      recurse(clip, rootNode);
    }

    _voronoiTreemap.convergenceRatio = function (_) {
      if (!arguments.length) {
        return convergenceRatio;
      }

      convergenceRatio = _;
      return _voronoiTreemap;
    };

    _voronoiTreemap.maxIterationCount = function (_) {
      if (!arguments.length) {
        return maxIterationCount;
      }

      maxIterationCount = _;
      return _voronoiTreemap;
    };

    _voronoiTreemap.minWeightRatio = function (_) {
      if (!arguments.length) {
        return minWeightRatio;
      }

      minWeightRatio = _;
      return _voronoiTreemap;
    };

    _voronoiTreemap.clip = function (_) {
      if (!arguments.length) {
        return clip;
      }

      //begin: use voronoiMap.clip() to handle clip/extent/size computation and borderline input (non-counterclockwise, non-convex, ...)
      _convenientReusableVoronoiMap.clip(_);
      //end: use voronoiMap.clip() to handle clip/extent/size computation
      clip = _convenientReusableVoronoiMap.clip();
      extent = _convenientReusableVoronoiMap.extent();
      size = _convenientReusableVoronoiMap.size();
      return _voronoiTreemap;
    };

    _voronoiTreemap.extent = function (_) {
      if (!arguments.length) {
        return extent;
      }

      //begin: use voronoiMap.extent() to handle clip/extent/size computation
      _convenientReusableVoronoiMap.extent(_);
      //end: use voronoiMap.clip() to handle clip/extent/size computation
      clip = _convenientReusableVoronoiMap.clip();
      extent = _convenientReusableVoronoiMap.extent();
      size = _convenientReusableVoronoiMap.size();
      return _voronoiTreemap;
    };

    _voronoiTreemap.size = function (_) {
      if (!arguments.length) {
        return size;
      }

      //begin: use voronoiMap.size() to handle clip/extent/size computation
      _convenientReusableVoronoiMap.size(_);
      //end: use voronoiMap.clip() to handle clip/extent/size computation
      clip = _convenientReusableVoronoiMap.clip();
      extent = _convenientReusableVoronoiMap.extent();
      size = _convenientReusableVoronoiMap.size();
      return _voronoiTreemap;
    };

    _voronoiTreemap.prng = function (_) {
      if (!arguments.length) {
        return prng;
      }

      prng = _;
      return _voronoiTreemap;
    };

    ///////////////////////
    /////// Private ///////
    ///////////////////////

    function recurse(clippingPolygon, node) {
      var simulation;

      //assign polygon to node
      node.polygon = clippingPolygon;

      if (node.height != 0) {
        //compute one-level Voronoi map of children
        simulation = voronoiMap
          .voronoiMapSimulation(node.children)
          .clip(clippingPolygon)
          .weight(function (d) {
            return d.value;
          })
          .convergenceRatio(convergenceRatio)
          .maxIterationCount(maxIterationCount)
          .minWeightRatio(minWeightRatio)
          .prng(prng)
          .stop();

        var state = simulation.state(); // retrieve the Voronoï map simulation's state

        //begin: manually launch each iteration until the Voronoï map simulation ends
        while (!state.ended) {
          simulation.tick();
          state = simulation.state();
        }
        //end: manually launch each iteration until the Voronoï map simulation ends

        //begin: recurse on children
        state.polygons.forEach(function (cp) {
          recurse(cp, cp.site.originalObject.data.originalData);
        });
        //end: recurse on children
      }
    }

    return _voronoiTreemap;
  }

  exports.voronoiTreemap = voronoiTreemap;

  Object.defineProperty(exports, '__esModule', { value: true });

}));