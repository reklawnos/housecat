var test = fn() {
    print("abc")
}

var foo = {
    print("this is a clip!")
    bar: fn() -> bagels {
        bagels = "test"
        print("woo!")
    }
    bar: fn(param1, param2) -> (bagel, butter) {
        bagel = "woo" + param1
        butter = "hoo!" + param2
    }
}

test()
foo()
