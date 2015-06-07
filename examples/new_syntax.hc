# any value can be a key in a clip
var scotsman = ${
    @ true: "yay I am a true scotsman"
    @ false: "no I am not a true scotsman"
    @ "is_true": false
    print_trueness: fn(self) {
        print(self.is_true)
    }
    i_am_an_ident: "this is a string value"
    
    # can directly use strings as keys
    @ "this is a string key": 123
    
    # can also use expressions
    @ "this_" + "key": "this value"  # assigned to key "this_key"
}

scotsman|print_trueness()  # prints "no I am not a true person", which is syntactic sugar for...
scotsman.print_trueness(scotsman)  # ...this equivalent statement.

### list constructor...
# var list = ["a", "b", "c"]
# ...which syntactic sugar for...
var list = ${
    @ 0: "a"
    @ 1: "b"
    @ 3: "c"
}

### javascript-like definition
var data_structure = ${
    name: "jacobs"
    occupation: "miner"
    # hobbies: [
    #     "eating",
    #     "sleeping",
    #     "underwater basket weaving"
    #]
}
