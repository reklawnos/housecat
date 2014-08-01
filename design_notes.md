Housecat Language
===================

Control Flow
-------------------
### If statements
If-then-else statements can be used like so:

    if x = 3 then
        print("x is 3!")
    else
        print("x is not 3...")
    end

The else statement can be dropped:
    
    if x = 3 then
        print("x is 3!")
    end

### While statements
While loops are written as:
    
    # prints out numbers 1 through 15
    def x: 0
    while x < 15 do
        x: x + 1
        print(x) 
    end

Clips
-------------------
Clips are defined with a `{...}` block. Clips have their own scope.

    def x: 3
    {
        def x: 6
        print(x) # prints "6"
    }() # play the anonymous clip
    print(x) # prints "3"

If something is not defined in a clip, the parent scopes are checked recursively:

    def test: {
        print(y)
    }
    def y: "bagels!"
    test() # prints "bagels!"

### Using clips like objects
Clips can essentially act like objects, and their fields can be accessed through dot notation:

    def c: {
        def x: "foo"
    }
    print(c.x) # prints "foo"
    c.x: "bar"
    print(c.x) # prints "bar"

The `clone` function creates a copy of an existing clip:

    def person: {
        def name: "foo"
        def speak: {
            print(name)
        }
    }
    
    def person1: clone(person)
    def person2: clone(person)
    person1.name: "jack"
    person2.name: "jill"
    person1.speak() # prints "jack"
    person2.speak() # prints "jill"

### Def
The `def` keyword marks a variable as being the "definitive" version for that clip. Each time a new `def` is made with the same variable, it is overridden. The reason for this is if there is functionality inside of the clip, we want to get at a particular definition of a variable.

When assigning to a variable in a clip, the assignment is stored in the last-most `def`. For example:

    def cl: {
        def x: 3
        x: x + 3
        def x: 7
    }
    print(cl.x) # prints "7"
    cl.x: 10
    print(cl.x) # prints "10"

Definitions can be added to a clip after it is assigned:
    
    def cl: {
        def x: 3
    }
    print(cl.x) # prints '3'
    # print(cl.y) # error: 'x' is not defined
    def cl.y: 4
    print(cl.y) # prints '4'

When accessing a variable, the last assignment at the point of access is used. For example:

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
    print(person.name)    # prints "Joe"
    person()              # prints "Jensen", then "Bagelman"
    person.name: "Alfred" # changes def of name
    print(person.name)    # prints "Joe"
    person()              # prints "Alfred", then "Bagelman"
    
This is useful for creating "smart" variables:

    def doubleVal: {
        def val: 0
        val: val * 2
    }
    def d: clone(doubleVal)
    print(d.val) # prints 0
    d.val: 2
    print(d.val) # prints 4

An attempted access or assignment of a variable that has not been defined in this scope or any ancestor scope results in an error.

    # print(butts) # error: 'butts' is not defined
    # butts: "face" # error: 'butts' is not defined
    def c: {
        print(butts)
    }
    # c() # error: 'butts' is not defined

Though adding `def butts: "farts"` before the first line would avoid all of the errors.

Dot notation specifies a particular scope that should be searched

All non-clip types are passed by value, while clips are passed by reference. A new, identical clip can be created by cloning it.

### Using clips like functions
By default, every clip includes a definition of `result`, equivalent to `def result: nil`. Playing a clip returns the value of `result`.

    def getBob: {
        def bob: {
            def name: "Bob"
        }
        result: bob
    }
    def myBob: getBob # a reference to 'bob' in the getBob clip
    print(myBob.name) # prints "bob"

Clips can take parameters that are passed into the scope of the clip.

    def printSaying: {(greeting, name)
        print(greeting + ", " + name)
    }
    printSaying("hello", "bagels") # prints 'hello, bagels'

This is functionally equivalent to:
    
    def printSaying: {
        def greeting: nil
        def name: nil
        print(greeting + ", " + name)
    }
    printSaying.greeting: "hello"
    printSaying.name: "bagels"
    printSaying() # prints 'hello, bagels'

Note that the parameters act like normal variable definitions. The only difference is that they can be set when they are played.

This can also be used for making a 'constructor:'

    def makePerson: {(name, age)
        def person: {
            def name: name
            def age: age
        }
        result: clone(person) # return a clone of 'person'
    }
    def bob: makePerson("bob", 25)
    print(bob.name) # prints "bob"
    print(bob.age) # prints 25

### Import
The `import` function allows a form of inheritance. All the fields inside of the imported clip are put inside of the scope where `import` is called. For example,

    def person: {
        def name: "foo"
        def speak: {
            print(name)
        }
    }
    
    person.name: "bagelmaster"
    
    def chef: {
        import(person)
        def name: "Guillaume"
        def cook: {
            print("I made food :3")
        }
    }

Is the same as:

    def chef: {
        def name: "foo"
        def speak: {
            print(name)
        }
        name: "bagelmaster"
        def name: "Guillaume"
        def cook: {
            print("I made food :3")
        }
    }
