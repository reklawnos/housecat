Housecat Language
=================

Closures
--------

###7/16

Basically everything within a `{...}` block is in a particular stack frame.

`{...} ` represents a given 'local' scope:

	var x: 3
	{
		var x: 6
		print(x) # prints "6"
	}() # call the anonymous closure
	print(x) # prints "3"	

`return` immediately exits the closure and the value of the closure becomes whatever was returned. For example:

        var x: {return 6} # x is 6


###7/17
Closures will essentially act like objets:

	var c: gi
