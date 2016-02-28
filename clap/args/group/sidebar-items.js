initSidebarItems({"struct":[["ArgGroup","`ArgGroup`s are a family of related arguments and way for you to express, \"Any of these arguments\". By placing arguments in a logical group, you can create easier requirement and exclusion rules instead of having to list each argument individually, or when you want a rule to apply \"any but not all\" arguments.For instance, you can make an entire `ArgGroup` required, this means that one (and *only* one) argument from that group must be present. Using more than one argument from an `ArgGroup` causes a parsing failure.You can also do things such as name an entire `ArgGroup` as a conflict or requirement for another argument, meaning any of the arguments that belong to that group will cause a failure if present, or must present respectively.Perhaps the most common use of `ArgGroup`s is to require one and *only* one argument to be present out of a given set. Imagine that you had multiple arguments, and you want one of them to be required, but making all of them required isn't feasible because perhaps they conflict with each other. For example, lets say that you were building an application where one could set a given version number by supplying a string with an option argument, i.e. `--set-ver v1.2.3`, you also wanted to support automatically using a previous version number and simply incrementing one of the three numbers. So you create three flags `--major`, `--minor`, and `--patch`. All of these arguments shouldn't be used at one time but you want to specify that *at least one* of them is used. For this, you can create a group.Finally, you may use `ArgGroup`s to pull a value from a group of arguments when you don't care exaclty which argument was actually used at runtime.ExamplesThe following example demonstrates using an `ArgGroup` to ensure that one, and only one, of the arguments from the specified group is present at runtime."]]});