function renderTeamFrequency(data) {
  console.log(data);
  let margin = {top: 30, right: 30, bottom: 30, left: 60},
    width = GraphConfig.width - margin.left - margin.right,
    height = GraphConfig.height - margin.top - margin.bottom;

  let svg = d3.select("#code-frequency")
    .append("svg")
    .attr("width", width + margin.left + margin.right)
    .attr("height", height + margin.top + margin.bottom)
    .append("g")
    .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

// add the x Axis
  let x = d3.scaleLinear()
    .domain([0, data.length])
    .range([0, width]);

  svg.append("g")
    .attr("transform", "translate(0," + height + ")")
    .call(d3.axisBottom(x));

  let max_added = d3.max(data, (d) => d.added);
  let max_deleted = d3.max(data, (d) => d.deleted);

// add the first y Axis
  let y1 = d3.scaleLinear()
    .range([height / 2, 0])
    .domain([0, max_added]);
  svg.append("g")
    .attr("transform", "translate(-20,0)")
    .call(d3.axisLeft(y1));

// add the first y Axis
  let y2 = d3.scaleLinear()
    .range([height / 2, height])
    .domain([0, -max_deleted]);
  svg.append("g")
    .attr("transform", "translate(-20,0)")
    .call(d3.axisLeft(y2));

  // Plot the area
  svg.append("path")
    .attr("class", "mypath")
    .datum(data)
    .attr("fill", "#69b3a2")
    .attr("opacity", ".6")
    .attr("stroke", "#000")
    .attr("stroke-width", 1)
    .attr("stroke-linejoin", "round")
    .attr("d", d3.line()
      .curve(d3.curveBasis)
      .x(function (d) {
        return x(d.index);
      })
      .y(function (d) {
        return y1(d.added);
      })
    );

// Plot the area
  svg.append("path")
    .attr("class", "mypath")
    .datum(data)
    .attr("fill", "#404080")
    .attr("opacity", ".6")
    .attr("stroke", "#000")
    .attr("stroke-width", 1)
    .attr("stroke-linejoin", "round")
    .attr("d", d3.line()
      .curve(d3.curveBasis)
      .x(function (d) {
        return x(d.index);
      })
      .y(function (d) {
        return y2(-d.deleted);
      })
    );

  // Handmade legend
  svg.append("circle").attr("cx", 290).attr("cy", 30).attr("r", 6).style("fill", "#69b3a2")
  svg.append("circle").attr("cx", 290).attr("cy", 60).attr("r", 6).style("fill", "#404080")
  svg.append("text").attr("x", 310).attr("y", 30).text("Added").style("font-size", "15px").attr("alignment-baseline", "middle")
  svg.append("text").attr("x", 310).attr("y", 60).text("Deleted").style("font-size", "15px").attr("alignment-baseline", "middle")
}
