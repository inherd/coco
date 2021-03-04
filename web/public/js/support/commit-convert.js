function commits_by_hours(data, options) {
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

function commits_by_authors(data) {
  let authors = [];
  let authorMap = {}
  for (let i = data.length - 1; i >= 0; i--) {
    let datum = data[i];
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

  let last_date = data[data.length - 1].date * 1000;
  let range = data[0].date * 1000;

  while (range <= last_date) {
    let day = standardFormatDate(range);
    dayMap[day] = {
      date: new Date(range),
      value: 0,
      total_line: 0,
      commits: []
    }

    range = range + 24 * 60 * 60 * 1000;
  }

  let total_line = 0;
  for (let i = 0; i < data.length; i++) {
    let datum = data[i];
    let day = formatDate(datum.date);
    total_line = total_line + datum.total_added - datum.total_deleted;
    if (dayMap[day]) {
      dayMap[day].value++;
      dayMap[day].commits.push(datum);
    } else {
      dayMap[day] = {
        date: new Date(datum.date * 1000),
        value: 1,
        commits: [datum]
      }
    }

    dayMap[day].total_line = total_line;
  }

  let result = [];
  for (let key in dayMap) {
    result.push(dayMap[key])
  }

  return result;
}

function commit_by_weeks(data) {
  let weekMap = {};

  let start_date = data[0].date * 1000;
  let last_date = data[data.length - 1].date * 1000;
  let range = data[0].date * 1000;
  let index = 1;
  while (range <= last_date) {
    weekMap[index] = {
      date: range,
      index: index,
      added: 0,
      deleted: 0,
      total: 0
    }

    range = range + 24 * 60 * 60 * 1000 * 7;
    index++;
  }

  for (let i = data.length - 1; i >= 0; i--) {
    let datum = data[i];
    let week = Math.floor((datum.date * 1000 - start_date) / (24 * 60 * 60 * 1000 * 7)) + 1;
    if (weekMap[week]) {
      weekMap[week].added = weekMap[week].added + datum.total_added;
      weekMap[week].deleted = weekMap[week].deleted + datum.total_deleted;
      weekMap[week].total = weekMap[week].added - weekMap[week].deleted
    } else {
      weekMap[week] = {
        date: weekMap[week].range,
        index: weekMap[week].index,
        added: datum.total_added,
        deleted: datum.total_deleted,
        total: datum.total_added - datum.total_deleted
      }
    }
  }

  let result = [];
  for (let key in weekMap) {
    result.push(weekMap[key])
  }

  return result;
}

function commits_by_users_with_range(data, range) {
  let usermap = {};
  for (let i = 0; i < data.length; i++) {
    let datum = data[i];
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
