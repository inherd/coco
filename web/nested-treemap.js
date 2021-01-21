
function renderNestedTreemap(originData) {
  const color = d3.scaleSequential([8, 0], d3.interpolateMagma);
  const format = d3.format(",d");
  const width = 1200;
  const height = 960;
  function hierarchy(data, delimiter = ".") {
    let root;
    const map = new Map;
    data.forEach(function find(data) {
      const {name, value} = data;
      if (map.has(name)) return map.get(name);
      const i = name.lastIndexOf(delimiter);
      map.set(name, data);
      if (i >= 0) {
        let found = find({name: name.substring(0, i), children: []});
        if (found.children) {
          found.children.push(data);
        } else {
          return data
        }
        data.name = name.substring(i + 1);
        if (!value) {
          data.value = 0;
        } else {
          data.value = value;
        }
      } else {
        root = data;
      }
      return data;
    });

    return root;
  }

  var dMap = {}

  for (let datum of originData) {
    // todo: add support for windows
    let path = datum.path
      .replace(".rs", "")
      .replace(".go", "")
      .replaceAll(/\//g, ".")
      .replace(/.src./g, ".")
      .replace(/src./g, "main.")

    dMap["root." + path] = {
      name: "root." + path,
      value: datum.code
    }
  }

  function treemap(data){
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

  var jdata = Object.values(dMap)
  var data = hierarchy(jdata);

  const root = treemap(data);

  const svg = d3.select("#nested-treemap").append("svg")
    .attr("viewBox", [0, 0, width, height])
    .style("font", "10px sans-serif");

  const shadow = DOM.uid("shadow");

  console.log(root);

  svg.append("filter")
    .attr("id", shadow.id)
    .append("feDropShadow")
    .attr("flood-opacity", 0.3)
    .attr("dx", 0)
    .attr("stdDeviation", 3);

  const node = svg.selectAll("g")
    .data(d3.group(root, d => {
      console.log(d);
      return d.height
    }))
    .join("g")
    .attr("filter", shadow)
    .selectAll("g")
    .data(d => d[1])
    .join("g")
    .attr("transform", d => `translate(${d.x0},${d.y0})`);

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
}
