function commit_to_hours_data(data, options) {
  let startDate = 0;
  if (options && options.before_month) {
    let one_month = 30;
    startDate = new Date(new Date().valueOf() - (options.before_month * one_month * 24 * 60 * 60 * 1000));
  }

  let hoursDate = [];
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
      if (date <= startDate) {
        continue;
      }

      dateMap[day][hour]++;
    }
  }

  for (let day in dateMap) {
    let day_hours = 24;
    for (let i = 1; i <= day_hours; i++) {
      hoursDate.push({
        day: day,
        hour: i,
        value: dateMap[day][i]
      })
    }
  }
  return hoursDate;
}

function commit_to_author_map(data) {
  let authors = [];
  let authorMap = {}
  for (let datum of data) {
    if (!authorMap[datum.email]) {
      authorMap[datum.email] = {
        name: datum.author,
        start: datum.date,
        end: datum.date,
        email: datum.email,
      }
    } else {
      authorMap[datum.email].start = datum.date;
    }
  }

  for (let author in authorMap) {
    authors.push(authorMap[author])
  }
  return authors;
}

function commit_by_days(data) {
  let dayMap = {};
  for (let datum of data.reverse()) {
    let day = formatDate(datum.date);
    if (dayMap[day]) {
      dayMap[day].value++;
    } else {
      dayMap[day] = {
        date: new Date(datum.date * 1000),
        value: 1,
      }
    }
  }

  let result = [];
  for (let key in dayMap) {
    result.push(dayMap[key])
  }

  return result;
}

function range_commits_by_users(data, range) {
  let usermap = {};
  for (let datum of data.reverse()) {
    if (!usermap[datum.email]) {
      usermap[datum.email] = {
        name: datum.author,
        email: datum.email,
        joinTime: datum.date,
        data: []
      }

      usermap[datum.email].data[0] = 1;
    } else {
      let week = (datum.date - usermap[datum.email].joinTime) / (24 * 60 * 60) / 7;
      let currentWeek = Math.round(week);
      if (currentWeek < range) {
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
  return usersData;
}
