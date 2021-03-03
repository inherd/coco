function renderCodeFlower(data, selector) {
  data = {
    links: [
      {"id": "Myriel", "group": 1},
      {"id": "CountessdeLo", "group": 1},
      {"id": "Geborand", "group": 1},
      {"id": "Champtercier", "group": 1},
    ],
    nodes: [
      {"source": "CountessdeLo", "target": "Myriel", "value": 1},
      {"source": "Geborand", "target": "Myriel", "value": 1},
      {"source": "Champtercier", "target": "Myriel", "value": 1},
    ]
  }

  let w = GraphConfig.width;
  let h = GraphConfig.height;

  const links = data.links.map(d => Object.create(d));
  const nodes = data.nodes.map(d => Object.create(d));

  d3.select(selector).selectAll("svg").remove();
  const svg = d3.select(selector).append("svg").attr("viewBox", [0, 0, w, h]);

  const simulation = d3.forceSimulation(nodes)
    .force("link", d3.forceLink().id(d => d.id))
    .force("charge", d3.forceManyBody())
    .force("center", d3.forceCenter(w / 2, h / 2));

  let color = function () {
    const scale = d3.scaleOrdinal(d3.schemeCategory10);
    return d => scale(d.group);
  }

  const link = svg.append("g")
    .attr("stroke", "#999")
    .attr("stroke-opacity", 0.6)
    .selectAll("line")
    .data(links)
    .join("line")
    .attr("stroke-width", d => Math.sqrt(d.value));

  const node = svg.append("g")
    .attr("stroke", "#fff")
    .attr("stroke-width", 1.5)
    .selectAll("circle")
    .data(nodes)
    .join("circle")
    .attr("r", 5)
    .attr("fill", color)

  node.append("title")
    .text(d => d.id);

  simulation.on("tick", () => {
    link
      .attr("x1", d => d.source.x)
      .attr("y1", d => d.source.y)
      .attr("x2", d => d.target.x)
      .attr("y2", d => d.target.y);

    node
      .attr("cx", d => d.x)
      .attr("cy", d => d.y);
  });
}
