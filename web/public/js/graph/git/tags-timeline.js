function renderTagsTimeline(originData) {
  let data = [];
  for (let i = originData.length - 1; i >= 0; i--) {
    let datum = originData[i]
    datum.date = datum.date * 1000;
    data.push(datum);
  }

  let yearOptions = buildYearOptions(data[0].date);

  d3.select("#tags-timeline-select")
    .selectAll('myOptions')
    .data(yearOptions)
    .enter()
    .append('option')
    .text(function (d) {
      return d;
    })
    .attr("value", function (d) {
      return d;
    })

  // When the button is changed, run the updateChart function
  d3.select("#tags-timeline-select").on("change", function (d) {
    let selectedOption = d3.select(this).property("value")
    let selectYear = new Date(selectedOption, 0, 1);
    let selectDate = data.filter((d) => d.date > selectYear);
    render(selectDate)
  })

  function render(selectData) {
    d3.select("#tags-timeline svg").remove();

    let margin = {top: 20, right: 20, bottom: 30, left: 50},
      width = GraphConfig.width - margin.left - margin.right,
      height = 500 - margin.top - margin.bottom;

    // create a tooltip
    let tooltip = d3.select("#tags-timeline")
      .append("div")
      .style("opacity", 0)
      .attr("class", "tooltip")

    let svg = d3.select("#tags-timeline").append("svg")
      .attr("width", width + margin.left + margin.right)
      .attr("height", height + margin.top + margin.bottom)
      .append("g")
      .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

    let x = d3.scaleLinear()
      .domain([0, selectData.length + 1])
      .range([0, width]);

    svg.append("g")
      .attr("transform", "translate(0," + height + ")")
      .call(d3.axisBottom(x));

    let first_year;
    if (selectData.length > 0) {
      first_year = new Date(selectData[0].date).getFullYear();
    } else {
      first_year = new Date().getFullYear();
    }

    let startDate = new Date(first_year, 0, 1);
    let y = d3.scaleTime()
      .domain([startDate, Date.now()])
      .range([height, 0]);

    svg.append("g")
      .call(d3.axisLeft(y));

    let line = d3.line()
      .x(function (d) {
        return x(d.date);
      })
      .y(function (d) {
        return y(d.index + 1);
      });

    let g = svg.append('g');

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
          return y(d.date)
        })
        .attr("x2", function (d) {
          return x(d.index + 1)
        })
        .attr("y2", function (d) {
          return y(d.date)
        })
        .attr("stroke", "black")
        .style("stroke-dasharray", ("3, 3"));

      tooltip.style("opacity", 1)
    }

    let mousemove = function (event, d) {
      tooltip
        .html("tag: " + d.name + "<br/>time: " + standardFormatDate(d.date) + "<br/> id: " + d.commit_id)
        .style("left", event.pageX + 20 + "px")
        .style("top", event.pageY + "px")
    }
    let mouseleave = function (event, d) {
      tooltip.style("opacity", 0);
      g.selectAll("#tooltip_path").remove();
    }

    let color = d3.scaleLinear()
      .domain([0, 5])
      .range(["#F00", "#000"]);

    g.selectAll("dot")
      .data(selectData)
      .enter()
      .append("circle")
      .attr("cx", function (d, i) {
        d.index = i;
        return x(i + 1);
      })
      .attr("cy", function (d) {
        return y(d.date);
      })
      .attr("r", 3)
      .style("fill", function (d) {
        return color(d.share_index);
      })
      .on("mouseover", mouseover)
      .on("mousemove", mousemove)
      .on("mouseleave", mouseleave)

    // limit display tags number
    if (selectData.length <= 50) {
      g.selectAll("dot")
        .data(selectData)
        .enter()
        .append("text")
        .text((d) => d.name)
        .attr("x", function (d) {
          return x(d.index + 1);
        })
        .attr("y", function (d) {
          return y(d.date) - 10;
        })
        .style("text-anchor", "middle")
    }
  }

  render(data);
}
