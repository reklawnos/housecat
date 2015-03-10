def person: {
    def name: "Jensen"
    def speak: {
        print(name)
    }
    speak()
    name: "Bagelman"
    speak()
    name: "Joe"
}
def get_somethings: fn() -> (retval1, retval2, retval3) {
    def retval1: "return 1"
    def retval2: "return 2"
    def retval3: "return 3"
} 

def get_somethings: fn() -> (retval) {
    def retval: "return"
}

print(person.name)    # prints "Joe"
person()              # prints "Jensen", then "Bagelman", then "Joe"
person.name: "Alfred" # changes def of name
print(person.name)    # prints "Joe"
person()              # prints "Alfred", then "Bagelman", then "Joe"
