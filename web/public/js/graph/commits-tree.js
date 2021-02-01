// based on https://github.com/alaingilbert/git2graph
// Copyright 2011,2012 Alain Gilbert <alain.gilbert.15@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
let renderCommitsTree = function (data) {
  let color = d3.scaleOrdinal(d3.schemeSet2).domain(data)
  let shaMap = {};
  let idx = 1;
  let column = 1;
  let lastSha = "";
  for (let datum of data) {
    if (!datum.commits) {
      return;
    }

    for (let commit of datum.commits) {
      let short = commit.substr(0, 6);

      if (!shaMap[short]) {
        shaMap[short] = {
          idx: idx,
          id: short,
          column: column,
          color: color(column),
          parents_paths: lastSha ? [] : [
            {
              "id": lastSha,
              path: [{
                x: lastSha.id,
                y: lastSha.column,
                type: 0,
              }]
            }
          ]
        };
        idx++;

      }

      lastSha = shaMap[short];
    }

    column++;
  }

  let tree = [];
  for (let value in shaMap) {
    tree.push(shaMap[value]);
  }

  let xGap = 11;
  let yGap = 20;
  let gap = 2 / 5 * yGap;
  let radius = 4;
  let shaMargin = 60;

  let svg = d3.select("#commits-tree").append("svg")
  svg.style('height', (tree.length + 1) * yGap + 2 * radius + 'px');
  svg.selectAll('*').remove();
  let sg = svg.append('g')
    .attr('transform', 'translate(0, ' + radius + ')')

  let lineFunction = d3.line()
    .x(function (d) {
      return d.x;
    })
    .y(function (d) {
      return d.y;
    })
    .curve(d3.curveMonotoneX)

  let commitGroup = sg.selectAll('commitGroup')
    .data(tree)
    .enter()
    .append('g');

  commitGroup.selectAll('lines')
    .data(function (d) {
      return d.parents_paths;
    })
    .enter()
    .append('path')
    .attr('d', function (path) {
      let d = [];
      for (let node of path.path) {
        let point = {x: 5 + node.x * xGap + shaMargin, y: 5 + node.y * yGap};
        switch (node.type) {
          case 1:
            point.y -= gap;
            break;
          case 2:
          case 3:
            point.y += gap;
            break;
        }
        d.push(point);
      }

      return lineFunction(d);
    })
    .attr('stroke-width', 2)
    .attr('fill', 'none')
    .attr('stroke', function (path) {
      return path.color || '#5aa1be';
    });

  sg.selectAll('commit')
    .data(tree)
    .enter()
    .append('circle')
    .attr('r', radius)
    .attr('fill', function (commit) {
      return commit.color || '#5aa1be';
    })
    .attr('stroke', 'black')
    .attr('cx', function (commit) {
      return 5 + commit.column * xGap + shaMargin;
    })
    .attr('cy', function (commit, idx) {
      return 5 + commit.idx * yGap;
    })
    .on('mouseover', function (commit) {
      console.log(commit.debug);
    });

  sg.selectAll('sha')
    .data(tree)
    .enter()
    .append('text')
    .attr('font-size', 12)
    .attr('x', function (commit) {
      return 0;
    })
    .attr('y', function (commit, idx) {
      return 5 + commit.idx * yGap;
    })
    .attr('alignment-baseline', 'middle')
    .attr('font-family', 'Consolas, "Liberation Mono", Menlo, Courier, monospace')
    .text(function (commit) {
      return commit.id.substr(0, 7);
    });
};
