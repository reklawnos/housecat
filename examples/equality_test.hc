var a = ${
  x: 1
  y: 2
  z: 3
}

var b = ${
  x: 1
  y: 2
  z: 3
}

var x = a
var y = b

if a == b do
  print("this shouldn't happen!")
else
  print("yay1")
end

if x == y do
  print("this also shouldn't happen!")
else
  print("yay2")
end

if a == a do
  print("this definitely should happen!")
else
  print("aww 1")
end

if a == x do
  print("so should this!")
else
  print("aww 2")
end

print(a.x)
print(x.x)
print(b.x)
print(y.x)
