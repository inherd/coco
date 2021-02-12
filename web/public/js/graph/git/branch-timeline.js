// based on https://observablehq.com/@tezzutezzu/world-history-timeline
let getTooltipContent = function (d) {
  return `<b>${d.name}</b>
<br/>
<b style="color:${d.color.darker()}">${d.name}</b>
<br/>
${formatDate(d.start)} - ${formatDate(d.end)}
`
}

let simplifyBranchName = function (name) {
  return name
    .replace("origin/", "")
  ;
}

function renderBranches(csv) {
  let data = csv.map(d => {
    return {
      ...d,
      start: d.start,
      end: d.end
    }
  }).sort((a, b) => a.start - b.start);

  let height = csv.length * 30;
  let width = GraphConfig.width;

  let margin = {
    top: 30,
    right: 150,
    bottom: 30,
    left: 30
  }

  let y = d3.scaleBand()
    .domain(d3.range(data.length))
    .range([0, height - margin.bottom - margin.top])
    .padding(0.2);

  let now = Date.now() / 1000;
  let x = d3.scaleLinear()
    .domain([d3.min(data, d => d.start), now])
    .range([0, width - margin.left - margin.right]);

  let createTooltip = function (el) {
    el
      .attr("class", "tooltip")
      .style("pointer-events", "none")
      .style("top", 0)
      .style("opacity", 0)
  }

  let getRect = function (d) {
    const el = d3.select(this);
    const sx = x(d.start);
    const w = x(d.end) - x(d.start);
    const isLabelRight = (sx > width / 2 ? sx + w < width : sx - w > 0);

    el.style("cursor", "pointer")

    el
      .append("rect")
      .attr("x", sx)
      .attr("height", y.bandwidth())
      .attr("width", w)
      .attr("fill", d.color);

    el
      .append("text")
      .text(simplifyBranchName(d.name))
      .attr("x", isLabelRight ? sx - 5 : sx + w + 5)
      .attr("y", 2.5)
      .attr("fill", "black")
      .style("text-anchor", isLabelRight ? "end" : "start")
      .style("dominant-baseline", "hanging");
  }

  let axisTop = d3.axisTop(x)
    .tickPadding(2)
    .tickFormat(formatDate);
  let axisBottom = d3.axisBottom(x)
    .tickPadding(2)
    .tickFormat(formatDate);

  let names = d3.group(data, d => simplifyBranchName(d.name));
  let color = d3.scaleOrdinal(d3.schemeSet2).domain(names)

  const svg = d3.select("#branch-timeline").append("svg")
    .attr("viewBox", `0 0 ${width} ${height}`)

  const g = svg.append("g").attr("transform", (d, i) => `translate(${margin.left} ${margin.top})`);
  const filteredData = data.sort((a, b) => a.start - b.start);
  filteredData.forEach(d => d.color = d3.color(color(d.name)))

  const groups = g
    .selectAll("g")
    .data(filteredData)
    .enter()
    .append("g")
    .attr("class", "civ")

  const tooltip = d3.select(document.createElement("div")).call(createTooltip);
  const line = svg.append("line").attr("y1", margin.top - 10).attr("y2", height - margin.bottom).attr("stroke", "rgba(0,0,0,0.2)").style("pointer-events", "none");

  groups.attr("transform", (d, i) => `translate(0 ${y(i)})`)
  groups
    .each(getRect)
    .on("mouseover", function (event, d) {
      d3.select(this).select("rect").attr("fill", d.color.darker())

      tooltip
        .style("opacity", 1)
        .html(getTooltipContent(d))
    })
    .on("mouseleave", function (event, d) {
      d3.select(this).select("rect").attr("fill", d.color)
      tooltip.style("opacity", 0)
    })

  svg
    .append("g")
    .attr("transform", (d, i) => `translate(${margin.left} ${margin.top - 10})`)
    .call(axisTop)

  svg
    .append("g")
    .attr("transform", (d, i) => `translate(${margin.left} ${height - margin.bottom})`)
    .call(axisBottom)

  svg.on("mousemove", function (event, d) {
    let [x, y] = d3.pointer(event);
    line.attr("transform", `translate(${x} 0)`);
    y += 20;
    if (x > width / 2) x -= 100;

    tooltip
      .style("left", x + "px")
      .style("top", y + "px")
  })

  let element = document.getElementById("branch-timeline");
  element.appendChild(svg.node());
  element.appendChild(tooltip.node());
  element.groups = groups;
}
