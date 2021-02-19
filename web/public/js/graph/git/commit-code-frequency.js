function renderCodeFrequency(data) {
  let margin = {top: 30, right: 30, bottom: 30, left: 80},
    width = GraphConfig.width - margin.left - margin.right,
    height = GraphConfig.height / 2 - margin.top - margin.bottom;

  let yearOptions = buildYearOptions(data[0].date);

  d3.select("#code-frequency-select")
    .selectAll('myOptions')
    .data(yearOptions)
    .enter()
    .append('option')
    .text(d => d)
    .attr("value", d => d)

  // When the button is changed, run the updateChart function
  d3.select("#code-frequency-select").on("change", function (d) {
    let selectedOption = d3.select(this).property("value")
    let selectYear = new Date(selectedOption, 0, 1);
    let selectDate = data.filter((d) => d.date > selectYear);
    render(selectDate)
  })

  function render(data) {
    d3.select("#code-frequency svg").remove();

    let svg = d3.select("#code-frequency")
      .append("svg")
      .attr("preserveAspectRatio", "xMinYMin meet")
      .attr("width", width + margin.left + margin.right)
      .attr("height", height + margin.top + margin.bottom)
      .attr("class", "code-frequency")
      .append("g")
      .attr("transform", "translate(" + margin.left + "," + margin.top + ")");

    let x = d3.scaleTime()
      .domain([data[0].date, data[data.length - 1].date])
      .range([0, width]);

    svg.append("g")
      .attr("transform", "translate(0," + height + ")")
      .call(d3.axisBottom(x));

    let max_added = d3.max(data, (d) => d.added);
    let max_deleted = d3.max(data, (d) => d.deleted);

    let y1 = d3.scaleLinear()
      .range([height / 2, 0])
      .domain([0, max_added]);

    svg.append("g")
      .attr("transform", "translate(-20,0)")
      .call(d3.axisLeft(y1));

    let y2 = d3.scaleLinear()
      .range([height / 2, height])
      .domain([0, max_deleted]);
    svg.append("g")
      .attr("transform", "translate(-20,0)")
      .call(d3.axisLeft(y2));

    svg.append("path")
      .attr("class", "addition")
      .datum(data)
      .attr("fill", "#2cbe4e")
      .attr("d", d3.area()
        .x(d => x(d.date))
        .y0(height / 2)
        .y1(d => y1(d.added))
      );

    svg.append("path")
      .attr("class", "deletion")
      .datum(data)
      .attr("fill", "#cb2431")
      .attr("d", d3.area()
        .x(d => x(d.date))
        .y0(height / 2)
        .y1(d => y2(d.deleted))
      );

    svg.append("circle").attr("cx", 290).attr("cy", 30).attr("r", 6).style("fill", "#2cbe4e")
    svg.append("circle").attr("cx", 290).attr("cy", 60).attr("r", 6).style("fill", "#cb2431")
    svg.append("text")
      .attr("x", 310)
      .attr("y", 30)
      .text("Added")
      .style("font-size", "15px")
      .attr("alignment-baseline", "middle")
    svg.append("text")
      .attr("x", 310)
      .attr("y", 60)
      .text("Deleted")
      .style("font-size", "15px")
      .attr("alignment-baseline", "middle")
  }

  render(data);
}
