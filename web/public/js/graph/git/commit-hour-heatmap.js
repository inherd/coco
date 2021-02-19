// based on: http://bl.ocks.org/ganezasan/dfe585847d65d0742ca7d0d1913d50e1
const renderHeatmapChart = function (data, id) {
  const margin = {top: 20, right: 0, bottom: 20, left: 40},
    width = (GraphConfig.width * 4 / 5) - margin.left - margin.right,
    height = GraphConfig.height / 2.5 - margin.top - margin.bottom,
    gridSize = Math.floor(width / 40),
    legendElementWidth = gridSize * 2,
    buckets = 8,
    colors = ["#ffffd9", "#edf8b1", "#c7e9b4", "#7fcdbb", "#41b6c4", "#1d91c0", "#225ea8", "#253494", "#081d58"], // alternatively colorbrewer.YlGnBu[9]
    days = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"],
    times = ["1a", "2a", "3a", "4a", "5a", "6a", "7a", "8a", "9a", "10a", "11a", "12a", "1p", "2p", "3p", "4p", "5p", "6p", "7p", "8p", "9p", "10p", "11p", "12p"];

  const svg = d3.select(id).append("svg")
    .attr("width", width + margin.left + margin.right)
    .attr("height", height + margin.top + margin.bottom)
    .append("g")
    .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

  const dayLabels = svg.selectAll(".dayLabel")
    .data(days)
    .enter().append("text")
    .text(function (d) {
      return d;
    })
    .attr("x", 0)
    .attr("y", (d, i) => i * gridSize)
    .style("text-anchor", "end")
    .attr("transform", "translate(-6," + gridSize / 1.5 + ")")
    .attr("class", (d, i) => ((i >= 0 && i <= 4) ? "dayLabel mono axis axis-workweek" : "dayLabel mono axis"));

  const timeLabels = svg.selectAll(".timeLabel")
    .data(times)
    .enter().append("text")
    .text((d) => d)
    .attr("x", (d, i) => i * gridSize)
    .attr("y", 0)
    .style("text-anchor", "middle")
    .attr("transform", "translate(" + gridSize / 2 + ", -6)")
    .attr("class", (d, i) => ((i >= 7 && i <= 16) ? "timeLabel mono axis axis-worktime" : "timeLabel mono axis"));

  const type = (d) => {
    return {
      day: +d.day,
      hour: +d.hour,
      value: +d.value
    };
  };

  const colorScale = d3.scaleQuantile()
    .domain([0, buckets - 1, d3.max(data, (d) => d.value)])
    .range(colors);

  const cards = svg.selectAll(".hour")
    .data(data, (d) => d.day + ':' + d.hour);

  cards.append("title");

  // create a tooltip
  let tooltip = d3.select(id)
    .append("div")
    .style("opacity", 0)
    .attr("class", "tooltip")
    .style("background-color", "white")
    .style("border", "solid")
    .style("border-width", "2px")
    .style("border-radius", "5px")
    .style("padding", "5px")

  // Three function that change the tooltip when user hover / move / leave a cell
  let mouseover = function (event, d) {
    tooltip.style("opacity", 1)
  }
  let mousemove = function (event, d) {
    tooltip
      .html("commits: " + d.value)
      .style("left", event.pageX + 20 + "px")
      .style("top", event.pageY + "px")
  }
  let mouseleave = function (event, d) {
    tooltip.style("opacity", 0)
  }

  cards.enter().append("rect")
    .attr("x", (d) => (d.hour - 1) * gridSize)
    .attr("y", (d) => (d.day - 1) * gridSize)
    .attr("rx", 4)
    .attr("ry", 4)
    .attr("class", "hour bordered")
    .attr("width", gridSize)
    .attr("height", gridSize)
    .on("mouseover", mouseover)
    .on("mousemove", mousemove)
    .on("mouseleave", mouseleave)
    .style("fill", colors[0])
    .merge(cards)
    .transition()
    .duration(1000)
    .style("fill", (d) => colorScale(d.value));

  cards.select("title").text((d) => d.value);

  cards.exit().remove();

  const legend = svg.selectAll(".legend")
    .data([0].concat(colorScale.quantiles()), (d) => d);

  const legend_g = legend.enter().append("g")
    .attr("class", "legend");

  legend_g.append("rect")
    .attr("x", (d, i) => legendElementWidth * i)
    .attr("y", height - 50)
    .attr("width", legendElementWidth)
    .attr("height", gridSize / 2)
    .style("fill", (d, i) => colors[i]);

  legend_g.append("text")
    .attr("class", "mono")
    .text((d) => "≥ " + Math.round(d))
    .attr("x", (d, i) => legendElementWidth * i)
    .attr("y", height + gridSize - 50);

  legend.exit().remove();
}

