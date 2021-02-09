function renderTagsTimeline(data) {
  data = data.reverse();

  let i = 0;
  data.forEach(function (d) {
    d.index = i;
    i++;
  });

  let margin = {top: 20, right: 20, bottom: 30, left: 50},
    width = GraphConfig.width - margin.left - margin.right,
    height = 500 - margin.top - margin.bottom;

  let svg = d3.select("#tags-timeline").append("svg")
    .attr("width", width + margin.left + margin.right)
    .attr("height", height + margin.top + margin.bottom)
    .append("g")
    .attr("transform",
      "translate(" + margin.left + "," + margin.top + ")");

  let x = d3.scaleLinear()
    .domain([0, data.length])
    .range([0, width]);

  svg.append("g")
    .attr("transform", "translate(0," + height + ")")
    .call(d3.axisBottom(x));

  let y = d3.scaleTime()
    .domain([data[0].date * 1000, Date.now()])
    .range([height, 0]);

  svg.append("g")
    .call(d3.axisLeft(y));

  svg.append('g')
    .selectAll("dot")
    .data(data)
    .enter()
    .append("circle")
    .attr("cx", function (d) {
      return x(d.index);
    })
    .attr("cy", function (d) {
      return y(d.date * 1000);
    })
    .attr("r", 3)
    .style("fill", "#69b3a2")
}
