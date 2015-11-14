var y = "it goes on"
var andonify = fn(s, i, b) -> res {
    res = s
    if false do
        return
    end
    while i > 0 do
        i = i - 1
        res = res + " and on"
        res
    end
}
y = andonify(y, 10, false)
print(y)
