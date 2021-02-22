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
let renderBranchTree = function (data, branches) {
  let tree = buildTree(data);

  let xGap = 11;
  let yGap = 20;
  let gap = 2 / 5 * yGap;
  let radius = 5;
  let shaMargin = 60;
  let commitMargin = 10;

  let width = GraphConfig.width;
  let svg = d3.select("#commits-tree").append("svg")
  svg.style('height', (tree.length + 1) * yGap + 2 * radius + 'px')
  svg.style('width', width);
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

  let defaultStrokeColor = '#5aa1be';

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
      return path.color || defaultStrokeColor;
    });

  sg.selectAll('commit')
    .data(tree)
    .enter()
    .append('circle')
    .attr('r', radius)
    .attr('fill', function (commit) {
      return commit.color || defaultStrokeColor;
    })
    .attr('stroke', 'black')
    .attr('cx', function (commit) {
      return 5 + commit.column * xGap + shaMargin;
    })
    .attr('cy', function (commit, idx) {
      return 5 + commit.idx * yGap;
    })
    .on('mouseover', function (event, commit) {
      console.log(commit);
    });

  sg.selectAll('sha')
    .data(tree)
    .enter()
    .append('text')
    .attr('x', function (commit) {
      return 0;
    })
    .attr('y', function (commit, idx) {
      return 5 + commit.idx * yGap;
    })
    .attr('alignment-baseline', 'middle')
    .text(function (commit) {
      return commit.id;
    });

  sg.selectAll('sha')
    .data(tree)
    .enter()
    .append('text')
    .attr('x', function (commit) {
      return 5 + branches.length * xGap + shaMargin + commitMargin;
    })
    .attr('y', function (commit, idx) {
      return 5 + commit.idx * yGap;
    })
    .attr('alignment-baseline', 'middle')
    .text(function (commit) {
      return formatDate(commit.date) + ", " + commit.author + ": " + commit.message;
    });
};

function buildTree(data) {
  let color = d3.scaleOrdinal(d3.schemeSet2).domain(data)
  let idx = 1;
  let column = 1;

  let shaMap = {};
  let branchMap = {};
  for (let datum of data) {
    shaMap[datum["commit_id"]] = datum;
    if (!branchMap[datum["branch"]]) {
      branchMap[datum["branch"]] = column;
      column++;
    }
  }

  for (let i = data.length - 1; i >= 0; i--) {
    let datum = data[i];
    let short = datum["commit_id"];
    let parent_hashes = [];

    shaMap[short] = {
      idx: idx,
      id: short,
      date: datum.date,
      author: datum.author,
      message: datum.message,
      branch: datum.branch,
      column: branchMap[datum["branch"]],
      color: color(branchMap[datum["branch"]]),
      parents_paths: parent_hashes
    };

    idx++;
  }

  for (let i = data.length - 1; i >= 0; i--) {
    let datum = data[i];
    let short = datum["commit_id"];
    let parent_hashes = [];
    for (let hash of datum["parent_hashes"]) {
      let item = shaMap[hash];
      if (item) {
        parent_hashes.push({
          id: hash,
          path: [
            {
              x: item.column,
              y: item.idx,
              type: 0,
            },
            {
              x: shaMap[short].column,
              y: shaMap[short].idx,
              type: 0
            },
          ]
        })
      }
    }
    shaMap[short].parents_paths = parent_hashes
  }

  let result = [];
  for (let value in shaMap) {
    result.push(shaMap[value]);
  }

  return result
}
