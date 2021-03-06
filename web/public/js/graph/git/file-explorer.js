function renderCodeExplorer(data, elementId) {
  let margin = {top: 20, right: 20, bottom: 50, left: 50};
  let focusHeight = 100;
  let width = GraphConfig.width - margin.left - margin.right;

  const rootNode = d3.hierarchy(data);
  rootNode.descendants().forEach((node) => {
    node.data.hierarchNode = node;
  });
  let maxDepth = 10;
  const allNodes = rootNode
    .descendants()
    .filter((d) => d.depth <= maxDepth)
    .filter(
      (d) => d.children === undefined || d.depth === maxDepth
    );

  function renderMainChart(nodes) {
    d3.select("svg#main-explorer").remove();

    const max = d3.quantile(nodes, 0.9975, d => {
      if (d.data.data && d.data.data.git && d.data.data.git.details.length) {
        return Math.abs(d.data.data.git.details.length)
      }
      return 0;
    });
    const average = Math.round(d3.quantile(nodes, 0.90, d => {
      if (d.data.data && d.data.data.git && d.data.data.git.details.length) {
        return Math.abs(d.data.data.git.details.length)
      }
      return 0;
    }));

    let color = d3.scaleLinear()
      .domain([0, average, +max])
      .range(['green', 'blue', 'red']);

    legend({
        color,
        title: "Daily commits",
        ticks: 10,
        tickFormat: function (d) {
          return d;
        }
      },
      d3.select(elementId))

    let svg = d3.select(elementId).append("svg")
      .attr("id", "main-explorer")
      .attr("width", GraphConfig.width)
      .attr("height", GraphConfig.width)
      .attr("viewBox", [-GraphConfig.width / 2, -GraphConfig.height / 2, GraphConfig.width, GraphConfig.height,]);

    const voronoi = svg.append("g").attr("transform", "translate(" + margin.left + "," + margin.top + ")");
    const labels = svg.append("g").attr("transform", "translate(" + margin.left + "," + margin.top + ")");

    let createTooltip = function (el) {
      el
        .attr("class", "tooltip")
        .style("pointer-events", "none")
        .style("top", 0)
        .style("opacity", 0)
    }
    const tooltip = d3.select(document.createElement("div")).call(createTooltip);
    let element = document.getElementById("file-explorer");
    element.append(tooltip.node());


    function fillFn(d) {
      if (d.data.data && d.data.data.git && d.data.data.git.details.length) {
        return color(d.data.data.git.details.length)
      } else {
        return color(0);
      }
    }

    voronoi.selectAll('path')
      .data(nodes)
      .enter()
      .append('path')
      .attr('d', d => `${d3.line()(d.data.layout.polygon)}z`)
      .attr('fill', fillFn)
      .attr("stroke", "#F5F5F2")
      .on("mouseover", function (event, d) {
        d3.select(this).attr("opacity", "0.5")
        let commits = 0;
        if (d.data.data && d.data.data.git) {
          commits = d.data.data.git.details.length;
        }
        tooltip
          .style("opacity", 1)
          .html(`<h2>${d.data.name}</h2>
<h4>${d.data.path}</h4>
<h4>line: ${d.data.value}</h4>
<h4>commits: ${commits}</h4>
`)
      })
      .on("mouseleave", function (event, d) {
        d3.select(this).attr("opacity", "1")
        tooltip.style("opacity", 0)
      })
      .on("click", function (event, d) {
        if (d.data.data && d.data.data.git) {
          renderSubGraph(d.data.data.git.details, "commit_day", "lines_added");
        }
      })
      .transition()
      .duration(1000)
      .attr("stroke-width", d => {
        if (d.data.layout.algorithm === "circlePack") return 0;
        return d.depth < 4 ? 4 - d.depth : 1;
      })

    svg.on("mousemove", function (event, d) {
      let [x, y] = d3.pointer(event);

      tooltip
        .style("left", x + GraphConfig.width / 2 + "px")
        .style("top", y + GraphConfig.width / 2 + "px")
    });

    labels.selectAll('text')
      .data(nodes.filter(d => {
        if (d.data.data && d.data.data.git) {
          if (d.data.data.git.details.length > average) {
            return true;
          }
        }

        return false;
      }))
      .enter()
      .append('text')
      .attr('class', d => `label-${d.id}`)
      .attr('text-anchor', 'middle')
      .attr("transform", d => {
        return "translate(" + [d.data.layout.center[0], d.data.layout.center[1] + 6] + ")"
      })
      .text(d => {
        if (d.data.data && d.data.data.git) {
          return d.data.name + ":" + d.data.data.git.details.length
        }
        return d.data.name;
      })
      .attr('cursor', 'default')
      .attr('pointer-events', 'none')
      .attr('fill', 'white')
  }

  renderMainChart(allNodes);

  function renderSubGraph(sub_data, x_key, y_key) {
    let width = GraphConfig.width - margin.left - margin.right;
    let height = 200 - margin.top - margin.bottom;

    d3.select("svg#sub_commit_graph").remove();
    let svg = d3.select(elementId)
      .append("svg")
      .attr("id", "sub_commit_graph")
      .attr("width", width + margin.left + margin.right)
      .attr("height", height + margin.top + margin.bottom)
      .append("g")
      .attr("transform",
        "translate(" + margin.left + "," + margin.top + ")");

    let x = d3.scaleTime()
      .domain(d3.extent(sub_data, function (d) {
        return d[x_key] * 1000;
      }))
      .range([0, width]);

    svg.append("g")
      .attr("transform", "translate(0," + height + ")")
      .call(d3.axisBottom(x));

    let y = d3.scaleLinear()
      .domain([0, d3.max(sub_data, function (d) {
        return +d[y_key];
      })])
      .range([height, 0]);

    svg.append("g")
      .call(d3.axisLeft(y));

    svg.append("path")
      .datum(sub_data)
      .attr("fill", "none")
      .attr("stroke", "steelblue")
      .attr("stroke-width", 1.5)
      .attr("d", d3.line()
        .x(function (d) {
          return x(d[x_key] * 1000)
        })
        .y(function (d) {
          return y(d[y_key])
        }))
  }

  let x = d3.scaleUtc()
    .domain(d3.extent(allNodes, d => {
      return d.data.data.git.creation_date
    }))
    .range([margin.left, width - margin.right])

  let y = d3.scaleLinear()
    .domain([0, d3.max(allNodes, d => {
      return d.data.data.git.details.length
    })])
    .range([200 - margin.bottom, margin.top])

  function focus(data_key) {
    let xAxis = (g) => g
      .attr("transform", `translate(0,${focusHeight - margin.bottom})`)
      .call(d3.axisBottom(x).ticks(width / 80).tickSizeOuter(0))

    let yAxis = (g, title) => g
      .attr("transform", `translate(${margin.left},0)`)
      .call(d3.axisLeft(y))
      .call(g => g.select(".domain").remove())
      .call(g => g.selectAll(".title").data([title]).join("text")
        .attr("class", "title")
        .attr("x", -margin.left)
        .attr("y", 10)
        .attr("fill", "currentColor")
        .attr("text-anchor", "start")
        .text(title))

    let area = (x, y) => d3.area()
      .defined(d => !isNaN(d[data_key]))
      .x(d => x(d.date))
      .y0(y(0))
      .y1(d => y(d[data_key]))

    const svg = d3.select(elementId)
      .append("svg")
      .attr("viewBox", [0, 0, width, focusHeight])
      .style("display", "block");

    const brush = d3.brushX()
      .extent([[margin.left, 0.5], [width - margin.right, focusHeight - margin.bottom + 0.5]])
      .on("brush", brushed)
      .on("end", brushended);

    const defaultSelection = [x(d3.utcYear.offset(x.domain()[1], -1)), x.range()[1]];

    svg.append("g")
      .call(xAxis, x, focusHeight);

    svg.append("path")
      .datum(data)
      .attr("fill", "#cce5df")
      .attr("d", area(x, y.copy().range([focusHeight - margin.bottom, 4])));

    const gb = svg.append("g")
      .call(brush)
      .call(brush.move, defaultSelection);

    function brushed({selection}) {
      if (selection) {
        svg.property("value", selection.map(x.invert, x).map(d3.utcDay.round));
        svg.dispatch("input");
      }
    }

    function brushended({selection}) {
      if (!selection) {
        gb.call(brush.move, defaultSelection);
      }
    }

    return svg.node();
  }

  let focus_chart = d3.select(focus("name"));
  console.log(focus_chart);

}
