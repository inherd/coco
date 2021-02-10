let count = 0;

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
