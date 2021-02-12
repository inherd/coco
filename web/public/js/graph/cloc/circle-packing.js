function renderPacking(originData) {
  let dMap = {};

  for (let datum of originData) {
    let path = CodeSupport.convertPath(datum.path)

    dMap["root." + path] = {
      name: "root." + path,
      value: datum.code
    }
  }

  let jdata = Object.values(dMap)
  let data = CodeSupport.hierarchy(jdata);

  let pack = function (data) {
    return d3.pack()
      .size([width, height])
      .padding(3)
      (d3.hierarchy(data)
        .sum(d => d.value)
        .sort((a, b) => b.value - a.value))
  }

  let width = GraphConfig.width;
  let height = width;

  let color = d3.scaleLinear()
    .domain([0, 5])
    .range(["hsl(152,80%,80%)", "hsl(228,30%,40%)"])
    .interpolate(d3.interpolateHcl)

  const root = pack(data);

  let focus = root;
  let view;

  const svg = d3.select("#circle-packing").append("svg")
    .attr("viewBox", `-${width / 2} -${height / 2} ${width} ${height}`);

  svg.style("display", "block")
    .style("background", color(0))
    .style("cursor", "pointer")
    .attr("text-anchor", "middle")
    .on("click", () => zoom(root));

  const node = svg.append("g")
    .selectAll("circle")
    .data(root.descendants().slice(1))
    .join("circle")
    .attr("fill", d => d.children ? color(d.depth) : "white")
    .attr("pointer-events", d => !d.children ? "none" : null)
    .on("mouseover", function () {
      d3.select(this).attr("stroke", "#000");
    })
    .on("mouseout", function () {
      d3.select(this).attr("stroke", null);
    })
    .on("click", (event, d) => focus !== d && (zoom(d), event.stopPropagation()))
    .on("contextmenu", (event, d) => {
      MenuSupport.createContextMenu(event, d, MenuSupport.defaultMenuItems, svg, {
        width: -width / 2,
        height: -height / 2
      });
      event.stopPropagation();
    })

  const label = svg.append("g")
    .attr("pointer-events", "none")
    .attr("text-anchor", "middle")
    .selectAll("text")
    .data(root.descendants())
    .join("text")
    .style("fill-opacity", d => d.parent === root ? 1 : 0)
    .style("display", d => d.parent === root ? "inline" : "none")
    .text(d => {
      if (!d.data.value) {
        return d.data.name
      }
      return d.data.name + ":" + d.data.value;
    });

  zoomTo([root.x, root.y, root.r * 2]);

  function zoomTo(v) {
    const k = width / v[2];

    view = v;

    label.attr("transform", d => `translate(${(d.x - v[0]) * k},${(d.y - v[1]) * k})`);
    node.attr("transform", d => `translate(${(d.x - v[0]) * k},${(d.y - v[1]) * k})`);
    node.attr("r", d => d.r * k);
  }

  function zoom(d) {
    focus = d;

    label
      .filter(function (d) {
        return d.parent === focus || this.style.display === "inline";
      })
      .transition(svg.transition()
        .duration(750)
        .tween("zoom", d => {
          const i = d3.interpolateZoom(view, [focus.x, focus.y, focus.r * 2]);
          return t => zoomTo(i(t));
        }))
      .style("fill-opacity", d => d.parent === focus ? 1 : 0)
      .on("start", function (d) {
        if (d.parent === focus) this.style.display = "inline";
      })
      .on("end", function (d) {
        if (d.parent !== focus) this.style.display = "none";
      });
  }
}
