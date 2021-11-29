ITOAGNSATI (I.Couldn't.Think.Of.A.Good.Name.So.Acronym.It.Is) pronounced ee-toe-ag-sat-ee is an incredibly
simple programming language, with the first version being done in a single night on 11/18/2021.


|-An example Hello World program-|
|const:a:"Hello World"           |
|print:a                         |

|-Command-|-Description----------------------------------------------------------------|-Syntax---------|
|const    |Declares an uneditable variable.                                            |const:a:10      |
|var      |Declares an editable variable.                                              |var:a:10        |
|print    |Prints the contents of a variable to the terminal.                          |print:a         |
|add      |Sets the value of a to a + b.                                               |add:a:b         |
|sub      |Sets the value of a to a - b.                                               |sub:a:b         |
|mult     |Sets the value of a to a * b.                                               |mult:a:b        |
|div      |Sets the value of a to a / b.                                               |div:a:b         |
|if       |Only does the following lines if a is true.                                 |if:a            |
|ifnz     |Only does the following lines if a != 0.                                    |ifnz:a          |
|while    |Loops while a is true.                                                      |while:a         |
|whilenz  |Loops while a != 0.                                                         |whilenz:a       |
|end      |Ends any statment, like for example an if                                   |end             |
|finish   |It returns from the entire program.                                         |finish          |
|set      |Sets a to the value of b if b's type is the same as origanal type of a      |set:a:b         |
|input    |Waits for input from the terminal and then sets a to it.                    |input:a         |
|parse    |Converts string a into a value and sets it to b.                            |parse:a:b       |

|-Commands that have yet to be implemented but may be in the future.-------------------|----------------|

SPECIAL COMMANDS:
These special commands may not work like most.

".to_owned()": Can be appended to the end of a string when its created to turn it into a full String from
               an str or &str.
               var:a:"STRING".to_owned()