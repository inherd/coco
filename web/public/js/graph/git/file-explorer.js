function renderCodeExplorer(data, elementId) {
  let margin = {top: 20, right: 20, bottom: 50, left: 50};
  let height = GraphConfig.height - margin.left - margin.right;
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

  let slider_svg = d3.select(elementId)
    .append("svg")
    .attr("width", GraphConfig.width)
    .attr("height", 100)
    .attr("id", "slider-chart");

  let moving = false;
  let currentValue = 0;


  let min_time = d3.min(allNodes, d => d.data.data.git.creation_date) * 1000;
  let max_time = d3.max(allNodes, d => d.data.data.git.last_update) * 1000;
  let startDate = min_time;

  let targetValue = width;
  let formatDate = d3.timeFormat("%Y %b %d");
  let formatDateIntoYear = d3.timeFormat("%Y");

  let playButton = d3.select("#play-button");
  let timer;
  playButton
    .on("click", function () {
      let button = d3.select(this);
      if (button.text() === "Pause") {
        moving = false;
        clearInterval(timer);
        // timer = 0;
        button.text("Play");
      } else {
        moving = true;
        timer = setInterval(step, 100);
        button.text("Pause");
      }
      console.log("Slider moving: " + moving);
    })

  function step() {
    update(x.invert(currentValue));
    currentValue = currentValue + (targetValue / 151);
    if (currentValue > targetValue) {
      moving = false;
      currentValue = 0;
      clearInterval(timer);
      playButton.text("Play");
      console.log("Slider moving: " + moving);
    }
  }

  let x = d3.scaleTime()
    .domain([min_time, max_time])
    .range([0, targetValue])
    .clamp(true);

  let slider = slider_svg.append("g")
    .attr("class", "slider")
    .attr("transform", "translate(" + margin.left + "," + 50 + ")");

  slider.append("line")
    .attr("class", "track")
    .attr("x1", x.range()[0])
    .attr("x2", x.range()[1])
    .select(function () {
      return this.parentNode.appendChild(this.cloneNode(true));
    })
    .attr("class", "track-inset")
    .select(function () {
      return this.parentNode.appendChild(this.cloneNode(true));
    })
    .attr("class", "track-overlay")
    .call(d3.drag()
      .on("start.interrupt", function () {
        slider.interrupt();
      })
      .on("start drag", function (event) {
        currentValue = event.x;
        update(x.invert(currentValue));
      })
    );

  slider.insert("g", ".track-overlay")
    .attr("class", "ticks")
    .attr("transform", "translate(0," + 18 + ")")
    .selectAll("text")
    .data(x.ticks(10))
    .enter()
    .append("text")
    .attr("x", x)
    .attr("y", 10)
    .attr("text-anchor", "middle")
    .text(function (d) {
      return formatDateIntoYear(d);
    });

  let handle = slider.insert("circle", ".track-overlay")
    .attr("class", "handle")
    .attr("r", 9);

  let label = slider.append("text")
    .attr("class", "label")
    .attr("text-anchor", "middle")
    .text(formatDate(startDate))
    .attr("transform", "translate(0," + (-25) + ")")

  function update(h) {
    // update position and text of label according to slider scale
    handle.attr("cx", x(h));
    label
      .attr("x", x(h))
      .text(formatDate(h));

    // filter data set and redraw plot
    let newData = allNodes.filter(function (d) {
      if (d.data.data && d.data.data.git && d.data.data.git) {
        return d.data.data.git.last_update * 1000 > h;
      }
      return true;
    });
    renderMainChart(newData, h);
  }

  let svg = d3.select(elementId).append("svg")
    .attr("id", "main-explorer")
    .attr("width", GraphConfig.width)
    .attr("height", GraphConfig.width)
    .attr("viewBox", [-GraphConfig.width / 2, -GraphConfig.height / 2, GraphConfig.width, GraphConfig.height,]);

  function filter_by_time(d, new_time) {
    return d.data.data.git.details.filter(d => {
      return d.commit_day * 1000 > new_time
    });
  }

  function renderMainChart(nodes, new_time) {
    const max = d3.quantile(nodes, 0.9975, d => {
      if (d.data.data && d.data.data.git && d.data.data.git.details.length) {
        let length = filter_by_time(d, new_time).length;
        return Math.abs(length)
      }
      return 0;
    });
    const average = Math.round(d3.quantile(nodes, 0.90, d => {
      if (d.data.data && d.data.data.git && d.data.data.git.details.length) {
        let length = filter_by_time(d, new_time).length;
        return length
      }
      return 0;
    }));

    let color = d3.scaleLinear()
      .domain([0, average, +max])
      .range(['green', 'blue', 'red']);

    function fillFn(d) {
      if (d.data.data && d.data.data.git && d.data.data.git.details.length) {
        let length = filter_by_time(d, new_time).length;
        return color(length)
      } else {
        return color(0);
      }
    }

    // legend({
    //     color,
    //     title: "Daily commits",
    //     ticks: 10,
    //     tickFormat: function (d) {
    //       return d;
    //     }
    //   },
    //   d3.select(elementId))

    const voronoi = svg.append("g").attr("transform", "translate(" + margin.left + "," + margin.top + ")");
    const labels = svg.append("g").attr("transform", "translate(" + margin.left + "," + margin.top + ")");

    let createTooltip = function (el) {
      el
        .attr("class", "tooltip")
        .style("pointer-events", "none")
        .style("top", 0)
        .style("opacity", 0)
    }
    d3.select(".tooltip").remove();
    const tooltip = d3.select(document.createElement("div")).call(createTooltip);
    let element = document.getElementById("file-explorer");
    element.append(tooltip.node());

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
          let filter_commits = filter_by_time(d, new_time);
          commits = filter_commits.length;
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
      // .transition()
      // .duration(1000)
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
          let length = filter_by_time(d, new_time).length;
          if (length > average) {
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
          let length = filter_by_time(d, new_time).length;
          return d.data.name + ":" + length
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
}
