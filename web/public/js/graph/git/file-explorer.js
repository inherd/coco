function renderCodeExplorer(freedom, data, elementId) {
  let margin = {top: 20, right: 20, bottom: 50, left: 50};
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

  const max = d3.quantile(allNodes, 0.9975, d => {
    if (d.data.data && d.data.data.git && d.data.data.git.details.length) {
      return Math.abs(d.data.data.git.details.length)
    }
    return 0;
  });
  let color = d3.scaleLinear()
    .domain([0, +max])
    .range(["#9be9a8", "red"])

  legend(
    {
      color,
      title: "Daily commits",
      ticks: 10,
      tickFormat: function (d) {
        return d;
      }
    },
    d3.select(elementId)
  )

  let svg = d3.select(elementId).append("svg")
    .attr("width", GraphConfig.width)
    .attr("height", GraphConfig.width)
    .attr("viewBox", [-GraphConfig.width / 2, -GraphConfig.height / 2, GraphConfig.width, GraphConfig.height,]);

  const voronoi = svg.append("g").attr("transform", "translate(" + margin.left + "," + margin.top + ")");
  const labels = svg.append("g").attr("transform", "translate(" + margin.left + "," + margin.top + ")");
  const pop_labels = svg.append("g").attr("transform", "translate(" + margin.left + "," + margin.top + ")");

  voronoi.selectAll('path')
    .data(allNodes)
    .enter()
    .append('path')
    .attr('d', d => `${d3.line()(d.data.layout.polygon)}z`)
    .style('fill', d => {
      if (d.data.data && d.data.data.git && d.data.data.git.details.length) {
        return color(d.data.data.git.details.length)
      } else {
        return color(0);
      }
    })
    .attr("stroke", "#F5F5F2")
    .on('mouseenter', d => {
      let label = labels.select(`.label-${d.id}`);
      label.attr('opacity', 1)
      let pop_label = pop_labels.select(`.label-${d.id}`);
      pop_label.attr('opacity', 1)
    })
    .on('mouseleave', d => {

    })
    .transition()
    .duration(1000)
    .attr("stroke-width", d => {
      if (d.data.layout.algorithm === "circlePack") return 0;
      return d.depth < 4 ? 4 - d.depth : 1;
    })

  labels.selectAll('text')
    .data(allNodes)
    .enter()
    .append('text')
    .attr('class', d => `label-${d.id}`)
    .attr('text-anchor', 'middle')
    .attr("transform", d => {
      return "translate(" + [d.data.layout.center[0], d.data.layout.center[1] + 6] + ")"
    })
    .text(d => {
      if (d.data.data) {
        return d.data.name + ":" + d.data.data.git.details.length
      }
      return d.data.name;
    })
    .attr('cursor', 'default')
    .attr('pointer-events', 'none')
    .attr('fill', 'white')

  pop_labels.selectAll('text')
    .data(allNodes)
    .enter()
    .append('text')
    .attr('class', d => {
      `label-${d.id}`
    })
    .attr("transform", d => {
      return "translate(" + [d.data.layout.center[0], d.data.layout.center[1] + 6] + ")"
    })
    .attr('text-anchor', 'middle')
    .text(d => d.data.value)
    .attr('opacity', 0)
    .attr('cursor', 'default')
    .attr('pointer-events', 'none')
    .attr('fill', 'black')
    .style('font-size', '12px');
}
