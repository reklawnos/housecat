def person: ${
    def name: "Jensen"
    def speak: {
        print(name)
    }
    speak()
    name: "Bagelman"
    speak()
    name: "Joe"
    self.something
}

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

def get_somethings: fn() -> (retval1, retval2, retval3) {
    retval1: "return 1"
    retval2: "return 2"
    retval3: "return 3"
}

x, y, z = get_somethings(

def get_something_2: fn() -> (retval) {
    retval: "return"
}

print(person.name)    # prints "Joe"
person()              # prints "Jensen", then "Bagelman", then "Joe"
person.name: "Alfred" # changes def of name
print(person.name)    # prints "Joe"
person()              # prints "Alfred", then "Bagelman", then "Joe"
