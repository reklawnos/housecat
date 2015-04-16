var y: "it goes on"
var andonify: fn(s, i, b) -> res {
    res: s
    while i > 0
        i: i - 1
        res: res + " and on"
    end
}
y: andonify(y, 1000, false)
y
