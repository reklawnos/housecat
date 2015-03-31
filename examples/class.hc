def adder: fn(num) -> ret {
    ret: ${
        def add_num: num
        def add: fn(a) -> ret {
            ret: add_num + a
        }
        def as_tuple: fn(a, b, c) -> ret {
            ret: (add(a), add(b), add(c))
        }
    }
}

var my_adder: adder(10)
print(my_adder.add(1))
print(my_adder.as_tuple(1, 2, 3))
