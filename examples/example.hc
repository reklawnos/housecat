# var i = import("import_test.hc")


var person = ${
    name: "Jensen"
    speak: fn(self){
        print(self.name)
    }
}

var test = fn() {
    print("abc")
}

var foo = ${
    bar1: fn() -> bagels {
        bagels = "test"
        print("woo!")
    }
    # bar2: fn(param1, param2) -> (bagel, butter) {
    #    bagel = "woo" + param1
    #    butter = "hoo!" + param2
    #}
}

#var get_somethings = fn() -> (retval1, retval2, retval3) {
#    retval1 = "return 1"
#    retval2 = "return 2"
#    retval3 = "return 3"
#}

# var x, var y, var z = get_somethings()

var get_something_2 = fn() -> (retval) {
    retval = "return"
}

test()
print(person.name)
person.name: "Alfred"
print(person.name)
person|speak()
print("bagel")


