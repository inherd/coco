// https://github.com/microsoft/TypeScriptSamples/blob/master/simple/animals.ts

class Animal {
  name : string
  constructor(name : string) {
    this.name = name;
  }
  move(meters) {
    console.log(this.name + " moved " + meters + "m.");
  }
}

class Snake extends Animal {
  move() {
    console.log("Slithering...");
    super.move(5);
  }
}

class Horse extends Animal {
  move() {
    console.log("Galloping...");
    super.move(45);
  }
}

var sam = new Snake("Sammy the Python")
var tom: Animal = new Horse("Tommy the Palomino")

sam.move()
tom.move(34)
