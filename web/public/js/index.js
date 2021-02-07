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
      commits: datum.commits,
    })
  }

  renderBranches(data)
});

d3.json("data/git-commits.json").then(function (data) {
  renderCommitsTree(data);

  renderMembersTimeline(commit_to_author_map(data));

  renderHeatmapChart("#hour-heatmap", commit_to_hours_data(data));
  renderHeatmapChart("#hour-heatmap-half-year", commit_to_hours_data(data, {before_month: 6}));
  renderHeatmapChart("#hour-heatmap-three-month", commit_to_hours_data(data, {before_month: 3}));

  let usermap = {};
  let datamap = [];
  let range = 30;

  for (let datum of data.reverse()) {
    if (!usermap[datum.email]) {
      usermap[datum.email] = {
        name: datum.author,
        email: datum.email,
        joinTime: datum.date,
        data: []
      }

      for (let i = 0; i < range; i++) {
        datamap[i] = {};
      }

      usermap[datum.email].data[0] = 1;
    } else {
      let week = (datum.date - usermap[datum.email].joinTime) / (24 * 60 * 60) / 7;
      let currentWeek = Math.round(week);
      if (currentWeek < 30) {
        if (!usermap[datum.email].data[currentWeek]) {
          usermap[datum.email].data[currentWeek] = 1;
        } else {
          usermap[datum.email].data[currentWeek]++;
        }
      }
    }
  }

  let usersData = [];
  for (let name in usermap) {
    for (let i = 0; i < range; i++) {
      if (!usermap[name].data[i]) {
        usermap[name].data[i] = 0;
      }
    }
    usersData.push(usermap[name]);
  }

  renderLearningCurve(usersData);
});
