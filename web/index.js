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
  }

}

d3.json("coco.json").then(function (json) {
  let data = json;
  // todo: refactor by select
  for (let datum of json) {
    if (datum.language === "Rust" || datum.language === "Go" || datum.language === "Java" || datum.language === "TypeScript") {
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
      format_start: datum.first_commit_str,
      format_end: datum.last_commit_str,
    })
  }

  renderBranches(data)
});

