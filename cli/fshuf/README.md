## fshuf - **v2.0**

Applies, modifies, or removes prefixes on every file in the current directory in order to "shuffle" the directories contents.

**Usage:**

`fshuf COMMAND [ PREFIX ]`

where COMMAND := { add | rem | mod | help }
	- add       : Adds a prefix to every file in the current directory.
	- rem       : Removes a prefix from every file in the current directory.
	- mod       : Modifies a prefix on every file in the current directory.
	- help      : Displays this menu.

where PREFIX ::= d | b | B
	- d         : Use a decimal number in the randomly generated prefixes.
	- b         : Use a binary number in the randomly generated prefixes.

**Example:**

`fshuf add bd`

