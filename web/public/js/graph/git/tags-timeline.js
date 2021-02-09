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

  // create a tooltip
  let tooltip = d3.select("#tags-timeline")
    .append("div")
    .style("opacity", 0)
    .attr("class", "tooltip")
    .style("background-color", "#ddd")
    .style("border", "solid")
    .style("border-width", "2px")
    .style("border-radius", "5px")
    .style("padding", "5px")

  let line = d3.line()
    .x(function (d) {
      return x(d.date * 1000);
    })
    .y(function (d) {
      return y(d.index);
    });

  // Three function that change the tooltip when user hover / move / leave a cell
  let mouseover = function (event, d) {

    g.selectAll("#tooltip_path")
      .data([d]).enter().append("line")
      .attr("id", "tooltip_path")
      .attr("class", "dot-line")
      .attr("d", line)
      .attr("x1", function (d) {
        return 0
      })
      .attr("y1", function (d) {
        return y(d.date * 1000)
      })
      .attr("x2", function (d) {
        return x(d.index)
      })
      .attr("y2", function (d) {
        return y(d.date * 1000)
      })
      .attr("stroke", "black")
      .style("stroke-dasharray", ("3, 3"));

    tooltip.style("opacity", 1)
  }

  let mousemove = function (event, d) {
    tooltip
      .html("tag: " + d.name + "<br/> time: " + formatDate(d.date))
      .style("left", event.pageX + 20 + "px")
      .style("top", event.pageY + "px")
  }
  let mouseleave = function (event, d) {
    tooltip.style("opacity", 0);
    g.selectAll("#tooltip_path").remove();
  }

  let g = svg.append('g');
  g.selectAll("dot")
    .data(data)
    .enter()
    .append("circle")
    .attr("cx", function (d) {
      return x(d.index);
    })
    .attr("cy", function (d) {
      return y(d.date * 1000);
    })
    .attr("r", 5)
    .style("fill", "#69b3a2")
    .on("mouseover", mouseover)
    .on("mousemove", mousemove)
    .on("mouseleave", mouseleave)
}
