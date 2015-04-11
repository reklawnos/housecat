person: ${
    name: "Jensen"
    speak: {
        print(name)
    }
    speak()
}

test: fn() {
    print("abc")
}

foo: ${
    bar1: fn() -> bagels {
        bagels: "test"
        print("woo!")
    }
    bar2: fn(param1, param2) -> (bagel, butter) {
        bagel: "woo" + param1
        butter: "hoo!" + param2
    }
}

get_somethings: fn() -> (retval1, retval2, retval3) {
    retval1: "return 1"
    retval2: "return 2"
    retval3: "return 3"
}

var x, var y, var z: get_somethings()

get_something_2: fn() -> (retval) {
    retval: "return"
}

test()
print(person.name)
person.name: "Alfred"
print(person.name)
person.speak()
print("bagel")

# any value can be a key in a clip
person: ${
    true: "yay I am a true person"
    false: "no I am not a true person"
    is_true: false
    print_trueness: {
        print(^.[is_true])
    }
}

# list constructor
