function renderNestedTreemap(originData) {
  let color = d3.scaleLinear()
    .domain([0, 4])
    .range(["hsl(0,0%,100%)", "hsl(197,73%,45%)"])
    .interpolate(d3.interpolateHcl);

  const format = d3.format(",d");
  const width = GraphConfig.width;
  const height = GraphConfig.height;
  const x = d3.scaleLinear().rangeRound([0, width]);
  const y = d3.scaleLinear().rangeRound([0, height]);

  var dMap = {}

  for (let datum of originData) {
    let path = CodeSupport.convertPath(datum.path);
    dMap["root." + path] = {
      name: "root." + path,
      path: datum.path,
      value: datum.code
    }
  }

  function treemap(data) {
    return d3.treemap()
      .size([width, height])
      .paddingOuter(8)
      .paddingTop(19)
      .paddingInner(2)
      .round(true)
      (d3.hierarchy(data)
        .sum(d => d.value)
        .sort((a, b) => b.value - a.value))
  }

  let data = CodeSupport.hierarchy(Object.values(dMap));

  const svg = d3.select("#nested-treemap").append("svg")
    .attr("id", "graphSvg")
    .attr("viewBox", [0.5, -30.5, width, height + 30])

  let group = svg.append("g")
    .call(render, treemap(data));

  function render(group, root) {
    const shadow = DOM.uid("shadow");

    svg.append("filter")
      .attr("id", shadow.id)
      .append("feDropShadow")
      .attr("flood-opacity", 0.3)
      .attr("dx", 0)
      .attr("stdDeviation", 3);

    const node = group.selectAll("g")
      .data(d3.group(root, d => {
        return d.height
      }))
      .join("g")
      .attr("filter", shadow)
      .selectAll("g")
      .data(d => d[1])
      .join("g")
      .attr("transform", d => `translate(${d.x0},${d.y0})`)
      .on("contextmenu", (event, d) => {
        MenuSupport.createContextMenu(event, d, MenuSupport.defaultMenuItems, svg);
      })

    node.append("title")
      .text(d => `${d.ancestors().reverse().map(d => d.data.name).join("/")}\n${format(d.value)}`);

    node.append("rect")
      .attr("id", d => (d.nodeUid = DOM.uid("node")).id)
      .attr("fill", d => color(d.height))
      .attr("width", d => d.x1 - d.x0)
      .attr("height", d => d.y1 - d.y0);

    node.append("clipPath")
      .attr("id", d => (d.clipUid = DOM.uid("clip")).id)
      .append("use")
      .attr("xlink:href", d => d.nodeUid.href);

    node.append("text")
      .attr("clip-path", d => d.clipUid)
      .selectAll("tspan")
      .data(d => d.data.name.split(/(?=[A-Z][^A-Z])/g).concat(format(d.value)))
      .join("tspan")
      .attr("fill-opacity", (d, i, nodes) => i === nodes.length - 1 ? 0.7 : null)
      .text(d => d);

    node.filter(d => d.children).selectAll("tspan")
      .attr("dx", 3)
      .attr("y", 13);

    node.filter(d => !d.children).selectAll("tspan")
      .attr("x", 3)
      .attr("y", (d, i, nodes) => `${(i === nodes.length - 1) * 0.3 + 1.1 + i * 0.9}em`);

    node.filter(d => d === root ? d.parent : d.children)
      .attr("cursor", "pointer")
      .on("click", (event, d) => d === root ? zoomout(root) : zoomin(d));
  }

  function position(group, root) {
    group.selectAll("g")
      .attr("transform", d => d === root ? `translate(0,-30)` : `translate(${x(d.x0)},${y(d.y0)})`)
      .select("rect")
      .attr("width", d => d === root ? width : x(d.x1) - x(d.x0))
      .attr("height", d => d === root ? 30 : y(d.y1) - y(d.y0));
  }

  function zoomin(d) {
    const group0 = group.attr("pointer-events", "none");
    const group1 = group = svg.append("g").call(render, d);

    x.domain([d.x0, d.x1]);
    y.domain([d.y0, d.y1]);

    svg.transition()
      .duration(750)
      .call(t => group0.transition(t).remove()
        .call(position, d.parent))
      .call(t => group1.transition(t)
        .attrTween("opacity", () => d3.interpolate(0, 1))
        .call(position, d));
  }

  function zoomout(d) {
    const group0 = group.attr("pointer-events", "none");
    const group1 = group = svg.insert("g", "*").call(render, d.parent);

    x.domain([d.parent.x0, d.parent.x1]);
    y.domain([d.parent.y0, d.parent.y1]);

    svg.transition()
      .duration(750)
      .call(t => group0.transition(t).remove()
        .attrTween("opacity", () => d3.interpolate(1, 0))
        .call(position, d))
      .call(t => group1.transition(t)
        .call(position, d.parent));
  }
}
