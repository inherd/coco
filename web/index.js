function renderPacking(originData) {

}

d3.json("coco.json").then(function (json) {
  let data = json;
  for (let datum of json) {
    if (datum.language === "Rust") {
      data = datum;
    }
  }

  renderPacking(data)
});

