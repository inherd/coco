function renderCodeExplorer(freedom, data, elementId) {
  let margin = {top: 20, right: 20, bottom: 50, left: 50};
  let width = GraphConfig.width - margin.left - margin.right;
  let height = GraphConfig.width - margin.top - margin.bottom;

  const rootNode = d3.hierarchy(data); // .sum(d => d.value);
  rootNode.descendants().forEach((node) => {
    node.data.hierarchNode = node;
  });
  let maxDepth = 3;
  const allNodes = rootNode
    .descendants()
    .filter((d) => d.depth <= maxDepth)
    .filter(
      (d) => d.children === undefined || d.depth === maxDepth
    );

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
    .style('fill', d => d.parent ? d.parent.color : d.color)
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
      return d.depth < 4 ? 4 - d.depth : 1;
    })

  labels.selectAll('text')
    .data(allNodes)
    .enter()
    .append('text')
    .attr('class', d => `label-${d.id}`)
    .attr('text-anchor', 'middle')
    .attr("transform", d => {
      // return "translate(" + [d.data.layout.polygon.site.x, d.data.layout.polygon.site.y + 6] + ")"
    })
    .text(d => {
      return d.data.path
    })
    .attr('opacity', function (d) {

    })
    .attr('cursor', 'default')
    .attr('pointer-events', 'none')
    .attr('fill', 'white')

  // pop_labels.selectAll('text')
  //   .data(allNodes.filter(d => d.depth === 2))
  //   .enter()
  //   .append('text')
  //   .attr('class', d => `label-${d.id}`)
  //   .attr('text-anchor', 'middle')
  //   // .attr("transform", d => "translate(" + [d.polygon.site.x, d.polygon.site.y + 25] + ")")
  //   .text(d => bigFormat(d.data.value))
  //   //.attr('opacity', d => d.data.key === hoveredShape ? 1 : 0)
  //   .attr('opacity', function (d) {
  //     if (d.data.key === hoveredShape) {
  //       return (1);
  //     } else if (d.data.value > 130000000) {
  //       return (1);
  //     } else {
  //       return (0);
  //     }
  //   })
  //
  //   .attr('cursor', 'default')
  //   .attr('pointer-events', 'none')
  //   .attr('fill', 'black')
  //   .style('font-size', '12px')
  //   .style('font-family', 'Montserrat');

}
