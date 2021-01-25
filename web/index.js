var count = 0;

function Id(id) {
  this.id = id;
  this.href = new URL(`#${id}`, location) + "";
}

Id.prototype.toString = function () {
  return "url(" + this.href + ")";
};

let DOM = {
  uid: function (name) {
    return new Id("O-" + (name == null ? "" : name + "-") + ++count)
  }
}

let CodeUtil = {
  convertPath: function (str) {
    // todo: multiple languages support
    return str
      .replace(".rs", "")
      .replace(".go", "")
      .replace(".java", "")
      .replace(".ts", "")
      .replace(".js", "")
      .replaceAll(/\//g, ".")
      .replace(/.src./g, ".")
      .replace(/src./g, "main.");
  },
  hierarchy: function (data, delimiter = ".") {
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
}


let Menu = {
  menuFactory: function(x, y, menuItems, data, svgId){
    d3.select(".contextMenu").remove();

    // Draw the menu
    d3.select(svgId)
      .append('g').attr('class', "contextMenu")
      .selectAll('tmp')
      .data(menuItems).enter()
      .append('g').attr('class', "menuEntry")
      .style({'cursor': 'pointer'});

    // Draw menu entries
    d3.selectAll(`.menuEntry`)
      .append('rect')
      .attr('x', x)
      .attr('y', (d, i) => { return y + (i * 30); })
      .attr('rx', 2)
      .attr('width', 150)
      .attr('height', 30)
      .on('click', (d) => { d.action(data) });

    d3.selectAll(`.menuEntry`)
      .append('text')
      .text((d) => { return d.title; })
      .attr('x', x)
      .attr('y', (d, i) => { return y + (i * 30); })
      .attr('dy', 20)
      .attr('dx', 45)
      .on('click', (d) => { d.action(data) });

    // Other interactions
    d3.select('body')
      .on('click', () => {
        d3.select(".contextMenu").remove();
      });
  },
  createContextMenu: function (event, d, menuItems, width, height, svgId) {
    Menu.menuFactory(event.pageX - width / 2, event.pageY - height / 1.5, menuItems, d, svgId);
    event.preventDefault();
  }
}

d3.json("cloc.json").then(function (json) {
  var data;
  var maxlen = 0;
  for (let i = 0; i < json.length; i++) {
    if (json[i].reports.length > maxlen) {
      maxlen = json[i].reports.length;
      data = json[i];
    }
  }

  renderPacking(data["reports"])
  renderNestedTreemap(data["reports"])
});

d3.json("branches.json").then(function (json) {
  let data = [];
  for (let datum of json) {
    data.push({
      name: datum.name,
      author: datum.author,
      start: datum.first_commit_date,
      end: datum.last_commit_date,
      format_start: datum.first_commit_str,
      format_end: datum.last_commit_str,
    })
  }

  renderBranches(data)
});

