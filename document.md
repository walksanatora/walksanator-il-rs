# Internal Keywords
<br>
- 00 - `CALL name [options...]` - used to denote a call to a function<br>
- 01 - `CALLN name [options...]` - used to denote a call to a native function<br>
- 02 - `STR constString` - used to refrence a string (string must be quoted if it contains spaces)<br>
- 03 - `VAR variableName` - refrence to a variable (needs a name or number)<br>
- 04 - `IF Value` - runs code if value is not NULL/`0`<br>
- 05 - `OP` - see [OP codes](#op-codes) for a list of options, used for comparison<br>
- 06 - `DEF` - opening of a function see [Defining a Function](#defining-a-function)<br>
- 07 - `CDEF` - opening of a class see [Defining a Class](#defining-a-class)<br>
- 08 - `EOF` - End Of File used to refrence the end of the file and determines<br>
- 09 - `ES` - End Statment used to close a (C)DEF or IF block<br>
- 0A - `LBL` `label` - label of where to jump to<br>
- 0B - `GOTO` `label` - jumps to a label<br>
- 0C - `MATH` `num` `op` `number` - performs math see<br> 
- 0D - `IMPORT` `STR name` - imports the scripe see [Importing another file](#importing-another-file) for more information<br>
- 0E - `RETURN` `value` - returns a value froma function/module<br>
- 0F - `SET` `variable` `value` - sets a value<br>
- 10 - `TYPE` `data` - returns a empty uninitiliazed version of that data<br>

# Math Operations
if a number is divided and is a decimal it gets rounded to the nearest whole<br>
if a number becomes negative it is 0<br>
- `0` var + a<br>
- `1` var - a<br>
- `2` var * a<br>
- `3` var / a<br>
- `4` var % a<br>
- `5` var ^ a<br>
<br>
<br>
`a` determines the output type so<br>
`MATH 0 100 "ohno"` would return the number 104 (sine string->number cast is length)<br>
`MATH 0 "ohno" 100` would return "ohno100"<br>


# OP codes
IF statments go through if the value is not `0`<br>
OP just is a builtin to make this easier<br>
```
IF OP # A B
```
- `0` not A  can be used to check if a variable is unset<br>
- `1` A and B<br>
- `2` A xor B<br>
- `3` A or B<br>
- `4` A < B<br>
- `5` A = B (checks if they are the same)<br>
- `6` A > B<br>
- `7` A = B (compares class)<br>
<br>
to allow OP codes on your classes give it a function like<br>
type and other_type are uninstantiated classes<br>
self and other are instances of a class
```
DEF op_0 type self other_type other
	RETURN 1 #return any number > 0 for true
ES
```
***note*** you cannot overwrite op 7 as that is handled by the intepreter directly<br>
<br>
you could also implement a cast function so you dont have to write the operations
```
DEF cast_number type self
	return 1
ES
DEF cast_str type self
	return ohno 
ES
```

cast default<br>
str -> num gets the length<br>
num -> str is just a string representation of the number<br>
class -> str the name of the class<br>
class -> num the number of values/functions in the class<br>
class -> class only if the left class has a `cast_<class>` where class is the name of the class <br>
# Defining a Function

function which takes then returns a string with unused number and class args
```
DEF function STR string class #create a function named function
	RETURN MATH 0 string class #also boldly assumes the class has a cast_str
EF #close the function
```

Functions can accept *less* arguments then specified but never more
values not send into a function default to 0
unless you prexix the name with `STR` which defaults to ""


# Defining a Class

```
CDEF class #create a new class called name

	STR string = "ohno" #class has a STR object names string
	number = 123 #class has a number named number
	TYPE deeper #class has a name version of it's self that may be initialised

	DEF name self STR string number
		SET self.string string #set the string value
		SET self.number number #set the number
		SET self.deeper TYPE self
	ES
ES
```

# Importing another file

main.ils
```
IMPORT "module" #or IMPORT "module.ils" same diffrence
IMPORT "moduel" "duel" #import moduel under the name duel

CALLN "print" CALL module.hello 
CALLN "print" CALL duel.time

EOF
```
module.ils
```
CDEF module
	DEF hello
		RETURN "HELLO WORLD!" 
	ES
ES
RETURN module
EOF
```
moduel.ils
```
CDEF module
	DEF time
		RETURN "TIME TO DUEL!!!"
	ES
ES
RETURN module
EOF
```

when importing a module the `EOF` of the file will return the module