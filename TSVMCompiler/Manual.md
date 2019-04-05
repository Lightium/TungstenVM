# Entry point
```
func Main
	code goes here
end func
```

# Functions

Functions start with func <name> and end with func end, ex.
```
func MyFunction
end func
```

# Instructions

- none
	- No operations.
- var <Name> <Type> <Operator> <Value>
	- Name = Any String you'd like to use as a name.
	- Type = IntType/DecType/StrType
	- Operator = =, +=, -=, *=, /= or %=
	- Value = Value corresponding to Type. ex. IntType = 1, DecType = 2.0, StrType = "abc def"
- print <Variable>
	- Prints the value of the variable. Ex. 
	  `var i IntType = 1
	   print i` prints "1".
- eq <Variable1> <Variable2>
	- Checks if the values of Variable1 and Variable2 are the same, and if they are, res is set to true.
- lss <Variable1, Type = IntType or DecType> <Variable2, Type = IntType or DecType>
	- Checks if the values of Variable1 is less than the one of Variable2, and if it is, res is set to true.
- gtr <Variable1, Type = IntType or DecType> <Variable2, Type = IntType or DecType>
	- Checks if the values of Variable1 is greater than the one of Variable2, and if it is, res is set to true.
- or <Variable1, Type = IntType> <Variable2, Type = IntType>
	- Converts Variable1 and Variable2 to booleans (0 = false, any other number = true) and applies the logical OR operator to them, and stores the result in res.
- and <Variable1, Type = IntType> <Variable2, Type = IntType>
	- Converts Variable1 and Variable2 to booleans (0 = false, any other number = true) and applies the logical AND operator to them, and stores the result in res.
- res <Variable>
	- Stores the result of res in Variable.
- jmp <Num>
	- Jumps to a position in a function. ex.
	```
	func Func
	none <= 0 is here
	jmp 0
	end func
	```
- jt <Num>
	- Does the same as jmp but only if res is true.
