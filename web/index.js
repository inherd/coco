var count = 0;

function Id(id) {
  this.id = id;
  this.href = new URL(`#${id}`, location) + "";
}

Id.prototype.toString = function() {
  return "url(" + this.href + ")";
};

let DOM = {
  uid: function (name) {
    return new Id("O-" + (name == null ? "" : name + "-") + ++count)
  }
}

d3.json("coco.json").then(function (json) {
  let data = json;
  for (let datum of json) {
    if (datum.language === "Rust" || datum.language === "Go") {
      data = datum;
    }
  }

  renderPacking(data["reports"])
  renderNestedTreemap(data["reports"])
});

d3.json("branches-coco.fixtures.json").then(function (json) {
  let data = [];
  for (let datum of json) {
    data.push({
      name: datum.name,
      author: datum.author,
      start: datum.first_commit_date,
      end: datum.last_commit_date,
    })
  }

  renderBranches(data)
});

