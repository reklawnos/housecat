Housecat Language
===================

Control Flow
-------------------
### If statements
If-then-else statements can be used like so:

    if x = 3
        print("x is 3!")
    else
        print("x is not 3...")
    end

The else statement can be dropped:
    
    if x = 3
        print("x is 3!")
    end

Defining something inside of an if statement will define it for the parent scope, while vars are constrained to the if statement's scope.

    def my_clip: {
        if true
            def x: 10
            var y: 20
        end
        print(x)  # prints "10"
        # print(y)  # error: y is not defined
    }
    print(my_clip.x)  # prints "10"

### While statements
While loops are written as:
    
    # prints out numbers 1 through 15
    var x: 0
    while x < 15
        x: x + 1
        print(x) 
    end

Clips
-------------------
Clips are defined using `[fn(<params>)]? [-> <return value>]? {...}`. Clips have their own scope.

    var x: 3
    {
        def x: 6
        print(x)  # prints "6"
    }()  # play the anonymous clip
    print(x)  # prints "3"


If something is not defined in a clip, the parent scopes are checked recursively:

    def test: {
        print(y)
    }
    var y: "bagels!"
    test()  # prints "bagels!"

### Using clips like objects
Clips can essentially act like objects, and their fields can be accessed through dot notation:

    def c: {
        def x: "foo"
    }
    print(c.x)  # prints "foo"
    c.x: "bar"
    print(c.x)  # prints "bar"

The `clone` builtin creates a copy of an existing clip:

    def person: {
        def name: "foo"
        def speak: {
            print(name)
        }
    }
    
    var person1: clone(person)
    var person2: clone(person)
    person1.name: "jack"
    person2.name: "jill"
    person1.speak()  # prints "jack"
    person2.speak()  # prints "jill"

### Def
The `def` keyword defines a variable as being a field on that clip.

    def cl: {
        def x: 3
        #def x: 7  # error: 'x' is already defined for this clip
    }
    print(cl.x)  # prints "7"
    cl.x: 10
    print(cl.x)  # prints "10"

Definitions can be added to a clip after it is created:
    
    def cl: {
        def x: 3
    }
    print(cl.x)  # prints '3'
    # print(cl.y)  # error: 'y' is not defined
    def cl.y: 4
    print(cl.y)  # prints '4'

Playing a clip can alter things:

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
    print(person.name)     # prints "Joe"
    person()               # prints "Jensen", then "Bagelman"
    person.name: "Alfred"  # changes def of name
    print(person.name)     # prints "Alfred"
    person()               # prints "Alfred", then "Bagelman"

An attempted access or assignment of a variable that has not been defined in this scope or any ancestor scope results in an error.

    # print(foo)  # error: 'foo' is not defined
    # foo: "bar"  # error: 'foo' is not defined
    def c: {
        print(foo)
    }
    # c()  # error: 'foo' is not defined

Though adding `def foo: "bar"` before the first line would avoid all of the errors.

Dot notation specifies a particular scope that should be searched

All non-clip types are passed by value, while clips are passed by reference. A new, identical clip can be created by cloning it.

### Using clips like functions
Clips can return values if they are specified in the clip definition.

    def get_bob: fn() -> bob {
        bob: {
            def name: "Bob"
        }
    }
    var my_bob: get_bob  # a reference to 'bob' in the get_bob clip
    print(my_bob.name)  # prints "bob"

Clips can take parameters that are passed into the scope of the clip.

    def print_saying: fn(greeting, name) {
        print(greeting + ", " + name)
    }
    print_saying("hello", "bagels")  # prints 'hello, bagels'

This is functionally equivalent to:
    
    def print_saying: {
        def greeting: nil
        def name: nil
        print(greeting + ", " + name)
    }
    print_saying.greeting: "hello"
    print_saying.name: "bagels"
    print_saying() # prints 'hello, bagels'

Note that the parameters act like normal variable definitions. The only difference is that they can be set when they are played.

This can also be used for making a 'constructor:'

    def make_person: fn(name, age) -> result {
        def person: {
            def name: name
            def age: age
        }
        def result: clone(person)  # return a clone of 'person'
    }
    var bob: make_person("bob", 25)
    print(bob.name)  # prints "bob"
    print(bob.age)  # prints 25

Clips can also return multiple values in the form of a tuple:

    def get_values: fn(param1, param2) -> (ret_val1, ret_val2) {
        def retval_1: "return this value, appended to " + param1
        def retval_2: "return this value and " + param2 + " too!"
    }
    # destructuring and definition in same statement
    var val_1, var val_2: get_values("this string!", "another string")
    
    # alternately
    var val_3: nil
    var val_4: nil
    val_3, val_4: get_values("this string!", "another string")

    # also alternately
    var tup: get_values("this string!", "another string")
    print(tup[0])  # prints "return this value, appended to this string!"
    print(tup[1])  # prints "return this value and another string too!"

You can break out of a clip's playback using `return`.

    def process_input: fn(input) -> (result, error) {
        if !is_valid(input)
            result: nil
            error: "input is invalid!"
            return 
        end
        result: "this input is muy bueno"
        error: nil
    }
    (var result, var error): process_input("invalid input")
    print(error)  # prints "input is invalid!"

### Import
The `import` function allows files to be imported as a clip.

    # bagel.hc:
    def has_poppyseeds: true

    # main.hc
    var bagel: import('bagel.hc')
    print(bagel.toppings)  # prints true

### `var`
Variables can be defined that are not exported out of the clip's scope using the `var` keyword. You can sort of think of this as a 'private' variable, but it does not persist for the life of the clip. It only lives as long as the time it takes to play the clip. These variables are only visible to the scope they were defined in and child scopes.

    def my_clip: fn() -> sum {
        def field_a: "this is a field!"
        var add: " but is it?"
        sum: field_a + add

        def get_add: fn() -> result {
            result: add
        }
    }
    print(my_clip.field_a)  # prints "this is a field!"
    # print(my_clip.add)  # error: 'add' is not defined
    print(my_clip())  # prints "this is a field! but is it?"
    # print(my_clip.get_add())  #error: 'add' is not defined
