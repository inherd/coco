// such as: https://github.com/inherd/coco/graphs/code-frequency
function renderTeamFrequency(data) {
  let margin = {top: 30, right: 30, bottom: 30, left: 60},
    width = 460 - margin.left - margin.right,
    height = 400 - margin.top - margin.bottom;

  let svg = d3.select("#code-frequency")
    .append("svg")
    .attr("width", width + margin.left + margin.right)
    .attr("height", height + margin.top + margin.bottom)
    .append("g")
    .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

// add the x Axis
  let x = d3.scaleLinear()
    .domain([-10, 15])
    .range([0, width]);
  svg.append("g")
    .attr("transform", "translate(0," + height + ")")
    .call(d3.axisBottom(x));

// add the first y Axis
  let y1 = d3.scaleLinear()
    .range([height / 2, 0])
    .domain([0, 0.12]);
  svg.append("g")
    .attr("transform", "translate(-20,0)")
    .call(d3.axisLeft(y1).tickValues([0.05, 0.1]));

// add the first y Axis
  let y2 = d3.scaleLinear()
    .range([height / 2, height])
    .domain([0, 0.12]);
  svg.append("g")
    .attr("transform", "translate(-20,0)")
    .call(d3.axisLeft(y2).ticks(2).tickSizeOuter(0));

// Compute kernel density estimation
  let kde = kernelDensityEstimator(kernelEpanechnikov(7), x.ticks(60))
  let density1 = kde(data.filter(function (d) {
    return d.type === "variable 1"
  }).map(function (d) {
    return d.value;
  }));

  let density2 = kde(data.filter(function (d) {
    return d.type === "variable 2"
  }).map(function (d) {
    return d.value;
  }))

// Plot the area
  svg.append("path")
    .attr("class", "mypath")
    .datum(density1)
    .attr("fill", "#69b3a2")
    .attr("opacity", ".6")
    .attr("stroke", "#000")
    .attr("stroke-width", 1)
    .attr("stroke-linejoin", "round")
    .attr("d", d3.line()
      .curve(d3.curveBasis)
      .x(function (d) {
        return x(d[0]);
      })
      .y(function (d) {
        return y1(d[1]);
      })
    );

// Plot the area
  svg.append("path")
    .attr("class", "mypath")
    .datum(density2)
    .attr("fill", "#404080")
    .attr("opacity", ".6")
    .attr("stroke", "#000")
    .attr("stroke-width", 1)
    .attr("stroke-linejoin", "round")
    .attr("d", d3.line()
      .curve(d3.curveBasis)
      .x(function (d) {
        return x(d[0]);
      })
      .y(function (d) {
        return y2(d[1]);
      })
    );

  // Handmade legend
  svg.append("circle").attr("cx", 290).attr("cy", 30).attr("r", 6).style("fill", "#69b3a2")
  svg.append("circle").attr("cx", 290).attr("cy", 60).attr("r", 6).style("fill", "#404080")
  svg.append("text").attr("x", 310).attr("y", 30).text("variable A").style("font-size", "15px").attr("alignment-baseline", "middle")
  svg.append("text").attr("x", 310).attr("y", 60).text("variable B").style("font-size", "15px").attr("alignment-baseline", "middle")

  // Function to compute density
  function kernelDensityEstimator(kernel, X) {
    return function (V) {
      return X.map(function (x) {
        return [x, d3.mean(V, function (v) {
          return kernel(x - v);
        })];
      });
    };
  }

  function kernelEpanechnikov(k) {
    return function (v) {
      return Math.abs(v /= k) <= 1 ? 0.75 * (1 - v * v) / k : 0;
    };
  }
}
