// based on https://observablehq.com/@tezzutezzu/world-history-timeline
let getTooltipContent = function (d) {
  return `<b>${d.civilization}</b>
<br/>
<b style="color:${d.color.darker()}">${d.region}</b>
<br/>
${formatDate(d.start)} - ${formatDate(d.end)}
`
}

function renderBranches(csv) {
  let data = csv.map(d => {
    return {
      ...d,
      start: +d.start,
      end: +d.end
    }
  }).sort((a, b) => a.start - b.start);

  let height = 1000;
  let width = 1000;

  let margin = {
    top: 30,
    right: 30,
    bottom: 30,
    left: 30
  }

  let y = d3.scaleBand()
    .domain(d3.range(data.length))
    .range([0, height - margin.bottom - margin.top])
    .padding(0.2);

  let x = d3.scaleLinear()
    .domain([d3.min(data, d => d.start), d3.max(data, d => d.end)])
    .range([0, width - margin.left - margin.right]);

  let createTooltip = function (el) {
    el
      .style("position", "absolute")
      .style("pointer-events", "none")
      .style("top", 0)
      .style("opacity", 0)
      .style("background", "white")
      .style("border-radius", "5px")
      .style("box-shadow", "0 0 10px rgba(0,0,0,.25)")
      .style("padding", "10px")
      .style("line-height", "1.3")
      .style("font", "11px sans-serif")
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
      .text(d.civilization)
      .attr("x", isLabelRight ? sx - 5 : sx + w + 5)
      .attr("y", 2.5)
      .attr("fill", "black")
      .style("text-anchor", isLabelRight ? "end" : "start")
      .style("dominant-baseline", "hanging");
  }

  let dataByTimeline = d3.nest().key(d => d.timeline).entries(data);
  let dataByRegion = d3.nest().key(d => d.region).entries(data);

  let formatDate = d => d < 0 ? `${-d}BC` : `${d}AD`;
  let axisTop = d3.axisTop(x)
    .tickPadding(2)
    .tickFormat(formatDate);
  let axisBottom = d3.axisBottom(x)
    .tickPadding(2)
    .tickFormat(formatDate);

  let regions = d3.nest().key(d => d.region).entries(data).map(d => d.key);
  let color = d3.scaleOrdinal(d3.schemeSet2).domain(regions)

  const svg = d3.select("#timeline").append("svg")
    .attr("viewBox", `-${width / 2} -${height / 2} ${width} ${height}`)

  const g = svg.append("g").attr("transform", (d, i) => `translate(${margin.left} ${margin.top})`);
  const filteredData = data.sort((a, b) => a.start - b.start);

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
    .on("mouseover", function (d) {
      d3.select(this).select("rect").attr("fill", d.color.darker())

      tooltip
        .style("opacity", 1)
        .html(getTooltipContent(d))
    })
    .on("mouseleave", function (d) {
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


  svg.on("mousemove", function (d) {

    let [x, y] = d3.mouse(this);
    line.attr("transform", `translate(${x} 0)`);
    y += 20;
    if (x > width / 2) x -= 100;

    tooltip
      .style("left", x + "px")
      .style("top", y + "px")
  })

  parent.appendChild(svg.node());
  parent.appendChild(tooltip.node());
  parent.groups = groups;

}

renderBranches([{
  "civilization": "Aegean civilization",
  "start": "-2000",
  "end": "-1200",
  "startLabel": "",
  "endLabel": "",
  "region": "Europe (and colonial offshoots)",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Age of pre-colonial civilization (Christian, Islamic, and traditional kingdoms)",
  "start": "650",
  "end": "1880",
  "startLabel": "",
  "endLabel": "",
  "region": "Sub-Saharan Africa",
  "timeline": "MEDIEVAL WORLD"
}, {
  "civilization": "Age of Turkic empires",
  "start": "500",
  "end": "1200",
  "startLabel": "",
  "endLabel": "",
  "region": "Middle East",
  "timeline": "MEDIEVAL WORLD"
}, {
  "civilization": "Age of united Caliphate",
  "start": "650",
  "end": "900",
  "startLabel": "",
  "endLabel": "",
  "region": "Middle East",
  "timeline": "MEDIEVAL WORLD"
}, {
  "civilization": "Ancient Andean region",
  "start": "-1000",
  "end": "500",
  "startLabel": "",
  "endLabel": "",
  "region": "pre-colonial Americas",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Ancient China",
  "start": "-2000",
  "end": "500",
  "startLabel": "",
  "endLabel": "",
  "region": "East Asia",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Ancient Steppe empires",
  "start": "-1000",
  "end": "500",
  "startLabel": "",
  "endLabel": "",
  "region": "the Steppe",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "British India",
  "start": "1800",
  "end": "1945",
  "startLabel": "",
  "endLabel": "WWII",
  "region": "South Asia",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Classic age of Mesoamerica",
  "start": "100",
  "end": "900",
  "startLabel": "",
  "endLabel": "",
  "region": "pre-colonial Americas",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Colonial Africa",
  "start": "1880",
  "end": "1980",
  "startLabel": "",
  "endLabel": "",
  "region": "Sub-Saharan Africa",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Colonial United States",
  "start": "1500",
  "end": "1776",
  "startLabel": "",
  "endLabel": "",
  "region": "Europe (and colonial offshoots)",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Early Islamic period",
  "start": "1200",
  "end": "1500",
  "startLabel": "",
  "endLabel": "",
  "region": "South Asia",
  "timeline": "MEDIEVAL WORLD"
}, {
  "civilization": "Early modern China",
  "start": "1500",
  "end": "1918",
  "startLabel": "",
  "endLabel": "WWI",
  "region": "East Asia",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Early modern Europe",
  "start": "1500",
  "end": "1800",
  "startLabel": "",
  "endLabel": "",
  "region": "Europe (and colonial offshoots)",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Early Nubian civilization",
  "start": "-2000",
  "end": "-1000",
  "startLabel": "",
  "endLabel": "",
  "region": "Sub-Saharan Africa",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Egyptian civilization",
  "start": "-3000",
  "end": "-550",
  "startLabel": "",
  "endLabel": "",
  "region": "Middle East",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "First Persian Empire",
  "start": "-550",
  "end": "-330",
  "startLabel": "",
  "endLabel": "",
  "region": "Middle East",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Formative age of Mesoamerica",
  "start": "-1500",
  "end": "100",
  "startLabel": "",
  "endLabel": "",
  "region": "pre-colonial Americas",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Formative Japan (age of Yamato rule)",
  "start": "500",
  "end": "800",
  "startLabel": "",
  "endLabel": "",
  "region": "East Asia",
  "timeline": "MEDIEVAL WORLD"
}, {
  "civilization": "Formative US",
  "start": "1776",
  "end": "1865",
  "startLabel": "",
  "endLabel": "",
  "region": "Europe (and colonial offshoots)",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Fractured Islamic world",
  "start": "900",
  "end": "2018",
  "startLabel": "",
  "endLabel": "present",
  "region": "Middle East",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Great power US",
  "start": "1865",
  "end": "1945",
  "startLabel": "",
  "endLabel": "WWII",
  "region": "Europe (and colonial offshoots)",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Greek age",
  "start": "-1200",
  "end": "0",
  "startLabel": "",
  "endLabel": "",
  "region": "Europe (and colonial offshoots)",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Heian age",
  "start": "800",
  "end": "1200",
  "startLabel": "",
  "endLabel": "",
  "region": "East Asia",
  "timeline": "MEDIEVAL WORLD"
}, {
  "civilization": "Imperial Japan",
  "start": "1870",
  "end": "1945",
  "startLabel": "",
  "endLabel": "WWII",
  "region": "East Asia",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Indian kingdom age",
  "start": "-500",
  "end": "1200",
  "startLabel": "",
  "endLabel": "",
  "region": "South Asia",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Indus civilization",
  "start": "-2500",
  "end": "-1500",
  "startLabel": "",
  "endLabel": "",
  "region": "South Asia",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Inter-Persian period",
  "start": "-330",
  "end": "200",
  "startLabel": "",
  "endLabel": "",
  "region": "Middle East",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Kush",
  "start": "-1000",
  "end": "300",
  "startLabel": "",
  "endLabel": "",
  "region": "Sub-Saharan Africa",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Medieval Andean region",
  "start": "500",
  "end": "1530",
  "startLabel": "",
  "endLabel": "",
  "region": "pre-colonial Americas",
  "timeline": "MEDIEVAL WORLD"
}, {
  "civilization": "Medieval China",
  "start": "500",
  "end": "1500",
  "startLabel": "",
  "endLabel": "",
  "region": "East Asia",
  "timeline": "MEDIEVAL WORLD"
}, {
  "civilization": "Medieval Europe",
  "start": "500",
  "end": "1500",
  "startLabel": "",
  "endLabel": "",
  "region": "Europe (and colonial offshoots)",
  "timeline": "MEDIEVAL WORLD"
}, {
  "civilization": "Mesopotamian civilization",
  "start": "-3500",
  "end": "-550",
  "startLabel": "",
  "endLabel": "",
  "region": "Middle East",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Modern Africa",
  "start": "1980",
  "end": "2018",
  "startLabel": "WWII",
  "endLabel": "present",
  "region": "Sub-Saharan Africa",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Modern Europe",
  "start": "1800",
  "end": "2018",
  "startLabel": "",
  "endLabel": "present",
  "region": "Europe (and colonial offshoots)",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Modern India",
  "start": "1945",
  "end": "2018",
  "startLabel": "",
  "endLabel": "present",
  "region": "South Asia",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Modern Japan",
  "start": "1945",
  "end": "2018",
  "startLabel": "",
  "endLabel": "present",
  "region": "East Asia",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Mongol Empire",
  "start": "1200",
  "end": "1300",
  "startLabel": "",
  "endLabel": "",
  "region": "the Steppe",
  "timeline": "MEDIEVAL WORLD"
}, {
  "civilization": "Mughal Empire",
  "start": "1500",
  "end": "1800",
  "startLabel": "",
  "endLabel": "",
  "region": "South Asia",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Peak of Aksum",
  "start": "300",
  "end": "650",
  "startLabel": "",
  "endLabel": "",
  "region": "Sub-Saharan Africa",
  "timeline": "MEDIEVAL WORLD"
}, {
  "civilization": "People's Republic of China",
  "start": "1945",
  "end": "2018",
  "startLabel": "WWII",
  "endLabel": "present",
  "region": "East Asia",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Postclassic age of Mesoamerica",
  "start": "900",
  "end": "1520",
  "startLabel": "",
  "endLabel": "",
  "region": "pre-colonial Americas",
  "timeline": "MEDIEVAL WORLD"
}, {
  "civilization": "Ptolemaic Egypt",
  "start": "-330",
  "end": "0",
  "startLabel": "",
  "endLabel": "",
  "region": "Middle East",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Republic of China",
  "start": "1918",
  "end": "1945",
  "startLabel": "",
  "endLabel": "",
  "region": "East Asia",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Roman > Byzantine Egypt",
  "start": "0",
  "end": "650",
  "startLabel": "",
  "endLabel": "",
  "region": "Middle East",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Roman Empire",
  "start": "0",
  "end": "500",
  "startLabel": "",
  "endLabel": "",
  "region": "Europe (and colonial offshoots)",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Roman Republic",
  "start": "-500",
  "end": "0",
  "startLabel": "",
  "endLabel": "",
  "region": "Europe (and colonial offshoots)",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Second Persian Empire",
  "start": "-200",
  "end": "650",
  "startLabel": "",
  "endLabel": "",
  "region": "Middle East",
  "timeline": "ANCIENT WORLD"
}, {
  "civilization": "Shogunate",
  "start": "1200",
  "end": "1870",
  "startLabel": "",
  "endLabel": "",
  "region": "East Asia",
  "timeline": "MEDIEVAL WORLD"
}, {
  "civilization": "Superpower US",
  "start": "1945",
  "end": "2018",
  "startLabel": "",
  "endLabel": "present",
  "region": "Europe (and colonial offshoots)",
  "timeline": "MODERN WORLD"
}, {
  "civilization": "Vedic age",
  "start": "-1500",
  "end": "-500",
  "startLabel": "",
  "endLabel": "",
  "region": "South Asia",
  "timeline": "ANCIENT WORLD"
}])
