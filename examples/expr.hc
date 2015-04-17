var y: 3
var andonify: fn(s, i, b) -> res {
    res: s
    while i > 0
        i: i - 1
        res: res + i
        res
    end
}
y: andonify(y, 10000, false)
y
