let formatDate = function (d) {
  let unix_time = d * 1000;
  return standardFormatDate(unix_time);
};

let standardFormatDate = function (d) {
  let date = new Date(d);
  let year = date.getUTCFullYear();
  let month = date.getUTCMonth() + 1;
  let day = date.getUTCDate();
  return year + "-" + month + "-" + day
};
