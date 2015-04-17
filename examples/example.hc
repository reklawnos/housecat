var person = ${
    name: "Jensen"
    speak: fn(self){
        self.name
    }
}

var test = fn() {
    "abc"
}

var foo = ${
    bar1: fn() -> bagels {
        bagels = "test"
        "woo!"
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
person.name
person.name: "Alfred"
person.name
person|speak()
"bagel"
