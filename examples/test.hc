import("path/\"to\"/file.hcat") # .hcat is appended

# Single line comments using pound signs
var doPrint: func(name) {
	var greeting: "hello" # strings are mutable
	var butts: false
	print(greeting + " " + name) # string concatenation
}


doPrint()


fn(param1, param2) -> (ret_val1, ret_val2) {
    def retval_1: "return this value, appended to " + param1
    def retval_2: "return this value and " + param2 + " too!"
}