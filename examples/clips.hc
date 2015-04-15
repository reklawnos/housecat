def test: fn() -> (iden) {
    print("abc")
}

def foo : {
    print("this is a clip!")
    def bar: fn() -> bagels {
        def bagels: "test"
        print("woo!")
    }
    bar: fn(param1, param2) -> (bagel, butter) {
        def bagel: "woo" + param1
        def butter: "hoo!" + param2
    }
}

test()
foo()
