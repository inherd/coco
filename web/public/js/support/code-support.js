let CodeSupport = {
  convertPath: function (str) {
    // todo: multiple languages support
    return str
      .replace(".rs", "")

      .replace(".cpp", "")
      .replace(".c", "")
      .replace(".h", "")

      .replace(".go", "")

      .replace(".java", "")
      .replace(".kt", "")
      .replace(".groovy", "")

      .replace(".ts", "")
      .replace(".js", "")
      .replace(/\//g, ".")
      .replace(/.src./g, ".")
      .replace(/src./g, "main.");
  },
  hierarchy: function (data, delimiter = ".") {
    let root;
    const map = new Map;
    data.forEach(function find(data) {
      const {name, value} = data;
      if (map.has(name)) return map.get(name);
      const i = name.lastIndexOf(delimiter);
      map.set(name, data);
      if (i >= 0) {
        let found = find({name: name.substring(0, i), children: []});
        if (found.children) {
          found.children.push(data);
        } else {
          return data
        }
        data.name = name.substring(i + 1);
        data.value = value;
      } else {
        root = data;
      }
      return data;
    });

    return root;
  }
}
