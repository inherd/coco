function renderCodeExplorer(freedom, data, elementId) {
  let margin = {top: 20, right: 20, bottom: 50, left: 50};
  let width = GraphConfig.width - margin.left - margin.right;
  let height = GraphConfig.width - margin.top - margin.bottom;

  let ellipse = d3
    .range(100)
    .map(i => [
      (width * (1 + 0.99 * Math.cos((i / 50) * Math.PI))) / 2,
      (height * (1 + 0.99 * Math.sin((i / 50) * Math.PI))) / 2
    ])

  let selectedYear = "2008";
  let bigFormat = d3.format(",.0f");

  let freedom_year = [];
  freedom.map(obj => {
    if (obj.year === selectedYear) {
      freedom_year.push({
        name: obj.countries,
        value: parseInt(obj.population, 10),
        region_simple: obj.region_simple
      });
    }
  });

  let some = calculateCodeLayout(data);
  console.log(some);
  let freedom_nest = d3.group(freedom_year, d => d.region_simple)
  let data_nested = {key: "freedom_nest", values: freedom_nest}

  function treemap(data) {
    return d3.treemap()
      (d3.hierarchy(data)
        .sum(d => {
          d.line = 0;
          if (d.data && d.data.git && d.data.git.details) {
            for (let datum of d.data.git.details) {
              d.line = d.line + datum.lines_added - datum.lines_deleted
            }
          }
          return d.line
        })
        .sum(d => d.line)
      )
  }

  let root = treemap(data);
  console.log(root);

  let population_hierarchy = d3.hierarchy(data_nested.values).sum(d => d.value);
  console.log(population_hierarchy);

  let regionColor = function (region) {
    let colors = {
      "Middle East and Africa": "#596F7E",
      "Americas": "#168B98",
      "Asia": "#ED5B67",
      "Oceania": "#fd8f24",
      "Europe": "#919c4c"
    };
    return colors[region];
  }

  let svg = d3.select(elementId).append("svg")
    .attr("width", GraphConfig.width)
    .attr("height", GraphConfig.width)
  svg
    .append("rect")
    .attr("width", "100%")
    .attr("height", "100%")
    .style("fill", "#F5F5F2");

  const voronoi = svg.append("g").attr("transform", "translate(" + margin.left + "," + margin.top + ")");
  const labels = svg.append("g").attr("transform", "translate(" + margin.left + "," + margin.top + ")");
  const pop_labels = svg.append("g").attr("transform", "translate(" + margin.left + "," + margin.top + ")");

  let seed = new Math.seedrandom(20);
  let voronoiTreeMap = d3.voronoiTreemap()
    .prng(seed)
    .clip(ellipse);

  function colorHierarchy(hierarchy) {
    if (hierarchy.depth === 0) {
      hierarchy.color = 'black';
    } else if (hierarchy.depth === 1) {
      hierarchy.color = regionColor(hierarchy.data[0]);
    } else {
      hierarchy.color = hierarchy.parent.color;
    }

    if (hierarchy.children) {
      hierarchy.children.forEach(child => colorHierarchy(child))
    }
  }

  voronoiTreeMap(population_hierarchy);
  colorHierarchy(population_hierarchy);

  let allNodes = population_hierarchy.descendants()
    .sort((a, b) => b.depth - a.depth)
    .map((d, i) => Object.assign({}, d, {id: i}));

  let hoveredShape = null;

  voronoi.selectAll('path')
    .data(allNodes)
    .enter()
    .append('path')
    .attr('d', d => "M" + d.polygon.join("L") + "Z")
    .style('fill', d => d.parent ? d.parent.color : d.color)
    .attr("stroke", "#F5F5F2")
    .attr("stroke-width", 0)
    .style('fill-opacity', d => d.depth === 2 ? 1 : 0)
    .attr('pointer-events', d => d.depth === 2 ? 'all' : 'none')
    .on('mouseenter', d => {
      let label = labels.select(`.label-${d.id}`);
      label.attr('opacity', 1)
      let pop_label = pop_labels.select(`.label-${d.id}`);
      pop_label.attr('opacity', 1)
    })
    .on('mouseleave', d => {
      let label = labels.select(`.label-${d.id}`);
      label.attr('opacity', d => d.data.value > 130000000 ? 1 : 0)
      let pop_label = pop_labels.select(`.label-${d.id}`);
      pop_label.attr('opacity', d => d.data.value > 130000000 ? 1 : 0)
    })
    .transition()
    .duration(1000)
    .attr("stroke-width", d => 7 - d.depth * 2.8)
    .style('fill', d => d.color);

  labels.selectAll('text')
    .data(allNodes.filter(d => d.depth === 2))
    .enter()
    .append('text')
    .attr('class', d => `label-${d.id}`)
    .attr('text-anchor', 'middle')
    .attr("transform", d => "translate(" + [d.polygon.site.x, d.polygon.site.y + 6] + ")")
    .text(d => d.data.key || d.data.name)
    .attr('opacity', function (d) {
      if (d.data.key === hoveredShape) {
        return (1);
      } else if (d.data.value > 130000000) {
        return (1);
      } else {
        return (0);
      }
    })

    .attr('cursor', 'default')
    .attr('pointer-events', 'none')
    .attr('fill', 'black')
    .style('font-family', 'Montserrat');

  pop_labels.selectAll('text')
    .data(allNodes.filter(d => d.depth === 2))
    .enter()
    .append('text')
    .attr('class', d => `label-${d.id}`)
    .attr('text-anchor', 'middle')
    .attr("transform", d => "translate(" + [d.polygon.site.x, d.polygon.site.y + 25] + ")")
    .text(d => bigFormat(d.data.value))
    //.attr('opacity', d => d.data.key === hoveredShape ? 1 : 0)
    .attr('opacity', function (d) {
      if (d.data.key === hoveredShape) {
        return (1);
      } else if (d.data.value > 130000000) {
        return (1);
      } else {
        return (0);
      }
    })

    .attr('cursor', 'default')
    .attr('pointer-events', 'none')
    .attr('fill', 'black')
    .style('font-size', '12px')
    .style('font-family', 'Montserrat');

}
