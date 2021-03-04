function renderCodeFlower(originData, selector) {
  let dMap = {};
  for (let datum of originData) {
    let path = CodeSupport.convertPath(datum.path)

    dMap["root." + path] = {
      name: "root." + path,
      size: datum.code
    }
  }

  let jdata = Object.values(dMap)
  let data = CodeSupport.hierarchy(jdata);

  let width = GraphConfig.width;
  let height = GraphConfig.height;
  let i = 0;
  let node, link;

  const root = d3.hierarchy(data);

  const svg = d3.select(selector).append('svg')
    .attr("viewBox", [0, 0, width, height])
    .call(d3.zoom().scaleExtent([1, 8]).on('zoom', zoomed))
    .append('g')
    .attr('transform', 'translate(40,0)');

  const simulation = d3.forceSimulation()
    .force('link', d3.forceLink().id(function (d) {
      return d.id;
    }))
    .force('charge', d3.forceManyBody().strength(-15).distanceMax(300))
    .force('center', d3.forceCenter(width / 2, height / 2))
    .on('tick', ticked)

  function update() {
    const nodes = flatten(root)
    const links = root.links()

    link = svg
      .selectAll('.link')
      .data(links, function (d) {
        return d.target.id
      })

    link.exit().remove()

    const linkEnter = link
      .enter()
      .append('line')
      .attr('class', 'link')
      .style('stroke', '#000')
      .style('opacity', '0.2')
      .style('stroke-width', 2)

    link = linkEnter.merge(link)

    node = svg
      .selectAll('.node')
      .data(nodes, function (d) {
        return d.id
      })

    node.exit().remove()


    let text = svg.append('svg:text')
      .attr('class', 'nodetext')
      .attr('dy', 0)
      .attr('dx', 0)
      .attr('text-anchor', 'middle');

    const nodeEnter = node
      .enter()
      .append('g')
      .attr('class', 'node')
      .attr('stroke', '#666')
      .attr('stroke-width', 2)
      .style('fill', color)
      .style('opacity', 1)
      .on('click', clicked)
      .call(d3.drag()
        .on('start', dragstarted)
        .on('drag', dragged)
        .on('end', dragended))
      .on("mouseover", function (event, d) {
        text.attr('transform', 'translate(' + d.x + ',' + d.y + ')')
          .text(d.data.name + ": " + d.data.size + " loc")
          .style('display', 'block');
      })
      .on("mouseout", function (d) {
        text.style('display', 'none');
      });

    nodeEnter.append('circle')
      .attr("r", function (d) {
        return Math.sqrt(d.data.size) / 10 || 4.5;
      })
      .style('text-anchor', function (d) {
        return d.children ? 'end' : 'start';
      })
      .text(function (d) {
        return d.data.name
      })

    node = nodeEnter.merge(node)
    simulation.nodes(nodes)
    simulation.force('link').links(links)
  }

  update();

  function sizeContain(num) {
    num = num > 1000 ? num / 1000 : num / 100
    if (num < 4) num = 4
    return num
  }

  function color(d) {
    return d._children ? "#51A1DC" // collapsed package
      : d.children ? "#51A1DC" // expanded package
        : "#F94B4C"; // leaf node
  }

  function radius(d) {
    return d._children ? 8
      : d.children ? 8
        : 4
  }

  function ticked() {
    link
      .attr('x1', function (d) {
        return d.source.x;
      })
      .attr('y1', function (d) {
        return d.source.y;
      })
      .attr('x2', function (d) {
        return d.target.x;
      })
      .attr('y2', function (d) {
        return d.target.y;
      })

    node
      .attr('transform', function (d) {
        return `translate(${d.x}, ${d.y})`
      })
  }

  function clicked(event, d) {
    if (!event.defaultPrevented) {
      if (d.children) {
        d._children = d.children;
        d.children = null;
      } else {
        d.children = d._children;
        d._children = null;
      }
      update()
    }
  }

  function dragstarted(event) {
    if (!event.active) simulation.alphaTarget(0.3).restart();
    event.subject.fx = event.subject.x;
    event.subject.fy = event.subject.y;
  }

  function dragged(event) {
    event.subject.fx = event.x;
    event.subject.fy = event.y;
  }

  function dragended(event) {
    if (!event.active) simulation.alphaTarget(0);
    event.subject.fx = null;
    event.subject.fy = null;
  }

  function flatten(root) {
    const nodes = []

    function recurse(node) {
      if (node.children) node.children.forEach(recurse)
      if (!node.id) node.id = ++i;
      else ++i;
      nodes.push(node)
    }

    recurse(root)
    return nodes
  }

  function zoomed(event) {
    svg.attr('transform', event.transform)
  }
}
