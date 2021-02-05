d3.json("data/cloc.json").then(function (json) {
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

d3.json("data/git.json").then(function (json) {
  let data = [];
  for (let datum of json) {
    data.push({
      name: datum.name,
      author: datum.author,
      start: datum.first_commit_date,
      end: datum.last_commit_date,
      format_start: datum.first_commit_str,
      format_end: datum.last_commit_str,
      commits: datum.commits,
    })
  }

  renderBranches(data)
});

d3.json("data/git-commits.json").then(function (data) {
  renderCommitsTree(data)
  renderHeatmapChart(commit_to_hour_date(data));
});
