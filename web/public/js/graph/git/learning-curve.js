// based on: https://bl.ocks.org/d3noob/ed0864ef6ec6af1e360917c29f4b08da
function renderLearningCurve(data) {
  // parse the date / time
  let parseTime = d3.timeParse("%d-%b-%y");

  data.forEach(function (d) {
    d.date = parseTime(d.date);
    d.close = +d.close;
    d.open = +d.open;
  });

  // set the dimensions and margins of the graph
  let margin = {top: 20, right: 20, bottom: 30, left: 50},
    width = 960 - margin.left - margin.right,
    height = 500 - margin.top - margin.bottom;

// set the ranges
  let x = d3.scaleTime().range([0, width]);
  let y = d3.scaleLinear().range([height, 0]);

// define the 1st line
  let valueline = d3.line()
    .x(function (d) {
      return x(d.date);
    })
    .y(function (d) {
      return y(d.close);
    });

// define the 2nd line
  let valueline2 = d3.line()
    .x(function (d) {
      return x(d.date);
    })
    .y(function (d) {
      return y(d.open);
    });

// append the svg obgect to the body of the page
// appends a 'group' element to 'svg'
// moves the 'group' element to the top left margin
  let svg = d3.select("#learning-curve").append("svg")
    .attr("width", width + margin.left + margin.right)
    .attr("height", height + margin.top + margin.bottom)
    .append("g")
    .attr("transform",
      "translate(" + margin.left + "," + margin.top + ")");

  // Scale the range of the data
  x.domain(d3.extent(data, function (d) {
    return d.date;
  }));
  y.domain([0, d3.max(data, function (d) {
    return Math.max(d.close, d.open);
  })]);

  // Add the valueline path.
  svg.append("path")
    .data([data])
    .attr("class", "line")
    .attr("d", valueline);

  // Add the valueline2 path.
  svg.append("path")
    .data([data])
    .attr("class", "line")
    .style("stroke", "red")
    .attr("d", valueline2);

  // Add the X Axis
  svg.append("g")
    .attr("transform", "translate(0," + height + ")")
    .call(d3.axisBottom(x));

  // Add the Y Axis
  svg.append("g")
    .call(d3.axisLeft(y));
}
