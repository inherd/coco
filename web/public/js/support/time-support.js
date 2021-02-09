let formatDate = function (d) {
  let date = new Date(d * 1000);
  let year = date.getUTCFullYear();
  let month = date.getUTCMonth() + 1;
  let day = date.getUTCDate();
  return year + "-" + month + "-" + day
};

let standardFormatDate = function (d) {
  let date = new Date(d);
  let year = date.getUTCFullYear();
  let month = date.getUTCMonth() + 1;
  let day = date.getUTCDate();
  return year + "-" + month + "-" + day
};
