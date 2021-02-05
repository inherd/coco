let formatDate = function (d) {
  let date = new Date(d * 1000);
  let year = date.getUTCFullYear();
  let month = date.getUTCMonth() + 1;
  let day = date.getUTCDate();
  return year + "-" + month + "-" + day
};

function commit_to_hour_date(data) {
  let hourDate = [];
  let dateMap = {1: {}, 2: {}, 3: {}, 4: {}, 5: {}, 6: {}, 7: {}};
  for (let datum of data) {
    let date = new Date(datum.date * 1000);
    let day = date.getDay() + 1;
    let hour = date.getHours() + 1;

    if (dateMap[day][hour] === undefined) {
      let day_hours = 24;
      for (let i = 1; i <= day_hours; i++) {
        dateMap[day][i] = 0;
      }

      dateMap[day][hour] = 1;
    } else {
      dateMap[day][hour]++;
    }
  }

  for (let day in dateMap) {
    let day_hours = 24;
    for (let i = 1; i <= day_hours; i++) {
      hourDate.push({
        day: day,
        hour: i,
        value: dateMap[day][i]
      })
    }
  }
  return hourDate;
}
