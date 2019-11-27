# Overview

Whilst attempting to build the text based adventure games, we've come into issues relating to borrow checking, when attempting to mutate multiple attributes of a data container at the same time, where we end up getting stuck with issues surrounding attempts to take multiple mutable references to the same instance.

Some approaches suggest the concept of an arena, a concept that provides abstractions to create references to items guaranteed to have the same lifetime, which we believe may be the solution we are looking for.

Other approaches include the use of unsafe code, which whilst it may solve the issue, seems like a very heavy handed approach.

## Links
Some sample information can be found here: https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/

## Exercise
Create a datastructure which contains a vector of some kind of children struct.

We must be able to collect a reference to two of the children instances from the vector and pass mutable references to them to a function which will mutate them both.
