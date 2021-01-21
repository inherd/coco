function Id(id) {
  this.id = id;
  this.href = new URL(`#${id}`, location) + "";
}

Id.prototype.toString = function () {
  return "url(" + this.href + ")";
};

function renderPacking(originData) {
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
        data.value = value;
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
    let path = datum.path.replace(".rs", "")
      .replaceAll(/\//g, ".")
      .replace(/.src./g, ".")
      .replace(/src./g, "main.")

    dMap["root." + path] = {
      name: "root." + path,
      value: datum.code
    }
  }

  var jdata = Object.values(dMap)
  var data = hierarchy(jdata);

  var pack = function (data) {
    return d3.pack()
      .size([width, height])
      .padding(3)
      (d3.hierarchy(data)
        .sum(d => d.value)
        .sort((a, b) => b.value - a.value))
  }

  var width = 1200;
  var height = width;
  var format = d3.format(",d")
  var color = d3.scaleLinear()
    .domain([0, 5])
    .range(["hsl(152,80%,80%)", "hsl(228,30%,40%)"])
    .interpolate(d3.interpolateHcl)

  const root = pack(data);

  let focus = root;
  let view;

  const svg = d3.select("#packing").append("svg")
    .attr("viewBox", `-${width / 2} -${height / 2} ${width} ${height}`)
    .style("display", "block")
    // .style("margin", "0 -14px")
    .style("background", color(0))
    .style("cursor", "pointer")
    .style("font", "14px sans-serif")
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
    .on("click", d => focus !== d && (zoom(d), d3.event.stopPropagation()));

  const label = svg.append("g")
    .style("font", "14px sans-serif")
    .attr("pointer-events", "none")
    .attr("text-anchor", "middle")
    .selectAll("text")
    .data(root.descendants())
    .join("text")
    .style("fill-opacity", d => d.parent === root ? 1 : 0)
    .style("display", d => d.parent === root ? "inline" : "none")
    .text(d => d.data.name);

  zoomTo([root.x, root.y, root.r * 2]);

  function zoomTo(v) {
    const k = width / v[2];

    view = v;

    label.attr("transform", d => `translate(${(d.x - v[0]) * k},${(d.y - v[1]) * k})`);
    node.attr("transform", d => `translate(${(d.x - v[0]) * k},${(d.y - v[1]) * k})`);
    node.attr("r", d => d.r * k);
  }

  function zoom(d) {
    const focus0 = focus;
    focus = d;

    const transition = svg.transition()
      .duration(d3.event.altKey ? 7500 : 750)
      .tween("zoom", d => {
        const i = d3.interpolateZoom(view, [focus.x, focus.y, focus.r * 2]);
        return t => zoomTo(i(t));
      });

    label
      .filter(function (d) {
        return d.parent === focus || this.style.display === "inline";
      })
      .transition(transition)
      .style("fill-opacity", d => d.parent === focus ? 1 : 0)
      .on("start", function (d) {
        if (d.parent === focus) this.style.display = "inline";
      })
      .on("end", function (d) {
        if (d.parent !== focus) this.style.display = "none";
      });
  }
}

d3.json("coco.json").then(function (json) {
  let data = json;
  for (let datum of json) {
    if (datum.language === "Rust") {
      data = datum;
    }
  }

  renderPacking(data["reports"])
});

