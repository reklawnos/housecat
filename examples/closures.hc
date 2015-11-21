let x = "bagel"
# x = "nooo this doesn't work"
let a = {
  print(x)
}
a()

let func = fn() -> ret {
  let x = "butter"
  ret = {
    print(x)
  }
}

let printer = func()

printer()

print(x)

let func2 = fn() -> (ret1, ret2) {
  var y = "butter"
  ret1 = {
    y = "bagel"
  }
  ret2 = {
    print(y)
  }
}
let a, let b = func2()

a()
b()
