var test = ${
    @ "hola": "test"
    @ 2: "two!"
    a: "thing"
}

print(test.hola)
print(test["hola"])
print(test.a)
print(test[2])
