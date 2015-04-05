def person: ${
    def name: "Jensen"
    def speak: {
        print(name)
    }
    speak()
}

def test: fn() {
    print("abc")
}

def foo: ${
    def bar1: fn() -> bagels {
        def bagels: "test"
        print("woo!")
    }
    def bar2: fn(param1, param2) -> (bagel, butter) {
        def bagel: "woo" + param1
        def butter: "hoo!" + param2
    }
}

def get_somethings: fn() -> (retval1, retval2, retval3) {
    retval1: "return 1"
    retval2: "return 2"
    retval3: "return 3"
}

var x, var y, var z: get_somethings()

def get_something_2: fn() -> (retval) {
    retval: "return"
}

test()
print(person.name)
person.name: "Alfred"
print(person.name)
person.speak()
print("bagel")
