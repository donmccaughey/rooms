Rust Rooms
==========

A simple text adventure written in Rust.

Overview
--------
This is a Rust learning project for me, so you may find some of the code
awkward and shortcuts may be taken here and there.  The program is a simple
terminal-based text adventure where you navigate a graph of "rooms".  The 
rooms graph is built by reading a list of rooms (graph vertices) and "doors" 
(graph edges) from a text file.  The `rooms.txt` file is an example.  
Currently the game world is read-only; my next big task is to add some mutable 
state to the graph of rooms.

Design
------
There are two modules, `rooms` and `main`.  The `rooms` module defines structs
`Room` and `Rooms` and enum `RoomsError`.  The `main` module contains the 
`main()` function and the game's run loop.

### struct Room
A `Room` consists of a name, description and four raw pointers to other 
`Room`s (north, south, east and west).  This kind of graph node structure is 
very natural in C but awkward in Rust.  It took a lot of thrashing about 
before I settled on the current approach (see the section on graphs in Rust 
below).  `Room` has a set of methods for building up the rooms graph and 
navigating it.

### struct Rooms
`Rooms` collects the graph of `Room` structs together.  `Rooms` contains a 
vector of boxed `Room` structs.  Each `Room` is boxed (i.e. heap allocated) so
that it has a constant address for the life of the graph; this is necessary
since a `Room` holds raw pointers to other `Room`s.  Using `Vec<Room>` instead
of `Vec<Box<Room>>` isn't guaranteed to work here, since each `Room`s address
would then point directly to the vector's internal memory block, which may
be reallocated and change address as the vector grows.  `Rooms` has a `new()`
method that builds a four room graph, and a `read()` method, that reads the
graph of rooms from a text file.  The `first_room()` method returns the first
room added to the graph in order to start the game.

### main()
The `main()` function implements the game loop.  In classic terminal fashion,
it reads a line of text from the command prompt, looks at the first letter of
the line and acts accordingly: `n` moves the player north, `q` quits the game,
etc.

Graphs in Rust
--------------
The natural way to build a graph in C (using pointers to other nodes) does not
work in Rust (at least not when using normal Rust references).  I tried 
several different methods before settling on the current approach, which feels
comfortable to me.  Here's an outline of some different approaches to graph
building in Rust:

### Vector Indices
Each node in the graph is an element in a vector, and edges are represented by
vector indices.  This approach has the advantage of staying withing Rust's
"safe" code zone, and may have some performance advantages if your graph is
very large, since `Vec` uses one large memory block internally to hold 
elements.  The disadvantage is that the burden of ensuring that edges hold 
valid indices is on you; adding a node anywhere but the end of the vector or 
removing a node can make edges invalid, leading to a mysteriously malformed 
graph or a runtime crash.  If you never modify your graph after it's built, 
this isn't really a problem. 

### Reference Counted Ref Cells
The `std::rc::Rc` class is a reference-counted smart pointer.  It's a
companion to `Box`.  If you're familiar with C++ smart pointers, `Box` is
analogous to `unique_ptr` while `Rc` is like `shared_ptr`.  By itself, `Rc`
won't allow you to build a graph; due to Rust's strict rule against multiple
mutable references, `Rc` only allows non-mutable access to its contents.
`RefCell` exists to overcome this problem.  A `RefCell` implements a runtime
check to prevent multiple simultaneous mutable references to its contents.
When you place your node in a `RefCell`, you can call `borrow_mut()` on the
cell when you need a mutable reference to the node inside the cell, even when 
you have an immutable reference to the cell.  Then you can put your `RefCell` 
in an `Rc` and weave together your graph.

Because `Rc` implements a *strong* reference, you are likely to create retain 
cycles with your graph, the bane of every reference counting scheme.  The 
standard library also included `std::rc::Weak`, a companion to `Rc` that 
implements a weak reference count (which unfortunately did not stabilize in 
time for the 1.0 release).  You can use `Weak` to avoid retain cycles.  One way 
to employ `Weak` is to make all your graph edges weak references (e.g. 
`Weak<RefCell<Node>>`) so that graph nodes don't pin each other in memory and 
use a separate node collection (e.g. `Vec<Rc<RefCell<Node>>>`) to provide 
ownership.  If your graph is intended to live for the lifetime of your 
program, you may not need to worry about retain cycles.  

Another issue to consider is that `RefCell` imposes some small runtime 
overhead when you call `borrow()` and `borrow_mut()` to access the interior 
data, which may be a concern for applications that work with very large graphs.

### Arena Allocation
If the nodes in your graph all have the same lifetime, you can allocate them
out of a common memory arena. This approach allows you to use normal Rust
references to build your graph and guarantees that when the arena is destroyed,
all nodes in your graph will be reclaimed.  [This article][1] talks about
arenas in more detail.  As of Rust 1.0, `libarena` is still unstable.  I have
not yet tried this approach myself.

### Unsafe Code and Raw Pointers
This is the approach that I settled on, which seems natural to me coming to 
Rust with lots of C/C++ experience.  Graph edges are mutable raw pointers
(`*mut Room`).  Nodes are boxed (`Box<Room>`), so they live on the heap and 
don't move around in memory.  Since raw pointers don't tell the Rust compiler
anything about the lifetime of objects they point to, the `Rooms` struct has
a vector (`Vec<Box<Room>>`) that acts as the owner of the nodes.




[1]: https://github.com/nrc/r4cppp/blob/master/graphs/README.md

