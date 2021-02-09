function buildYearOptions(date) {
  let startDate = new Date(date);
  let startYear = startDate.getFullYear();
  let currentYear = new Date().getFullYear();

  let yearOptions = [];
  for (let i = startYear; i <= currentYear; i++) {
    yearOptions.push(i);
  }
  return yearOptions;
}

function renderTagsTimeline(data) {
  data = data.reverse();
  data.forEach(function (d) {
    d.date = d.date * 1000;
  });

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
    let selectYear = new Date(selectedOption, 0, 0, 0, 0, 0, 0);
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
      .style("background-color", "#ddd")
      .style("border", "solid")
      .style("border-width", "2px")
      .style("border-radius", "5px")
      .style("padding", "5px")

    let svg = d3.select("#tags-timeline").append("svg")
      .attr("width", width + margin.left + margin.right)
      .attr("height", height + margin.top + margin.bottom)
      .append("g")
      .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

    let x = d3.scaleLinear()
      .domain([0, selectData.length])
      .range([0, width]);

    svg.append("g")
      .attr("transform", "translate(0," + height + ")")
      .call(d3.axisBottom(x));

    let startDate = selectData[0].date;
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
        return y(d.index);
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
          return x(d.index)
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
      .domain([0, 2])
      .range(["#00F", "#F00"]);

    g.selectAll("dot")
      .data(selectData)
      .enter()
      .append("circle")
      .attr("cx", function (d, i) {
        d.index = i;
        return x(i);
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
    if (selectData.length <= 20) {
      g.selectAll("dot")
        .data(selectData)
        .enter()
        .append("text")
        .text((d) => d.name)
        .attr("x", function (d) {
          return x(d.index);
        })
        .attr("y", function (d) {
          return y(d.date) - 10;
        })
        .style("text-anchor", "middle")
        .style("font-size", "12px")
    }
  }

  render(data);
}
