In Caldo we have:

- cells

- genes in cells

- various input/output ports

- materials in cells

Components:

- chemistry

- wall

- location

- velocity

- gene

- processor

Simplifications leading to performance and simplicity:

A cell contains:
4 processors of each
32 data stack
16 genes of each 32 instructions

Fixed gene size; 32 instructions
Fixed stack size
Fixed amount of genes; 16

A replicator in a very simple stack based language:

== 0, Main

=0
ReadCell // set the read cell to self
=1
WriteCell // set the write cell to above us

Zero
SetLoop
Dup
=1
Call
Dup2
=16
Eq
Not
Loop
// spawn processor on write cell
Spawn

== 1 copy gene, invoked with gene id on stack

Dup
=2 // READ
Call
=3 // WRITE
Call

== 2 READ
Zero

SetLoop

Dup2
Read

=1
Add

Dup
=31
Eq
Not
Loop

Drop2

== 3 WRITE
Zero

SetLoop

Dup2
Write

=1
Add

Dup
=31
Eq
Not
Loop

Drop2

Running instructions take energy. Copying instructions requires enough
materials.

It's possible to write into another gene too, not just read

Call consumes top of stack, tries to find processor with that id (within a range), if not, immediately returns, otherwise processor moves there

There needs to be a namespace system: within a cell, and to another cell
So reading and writing needs to be cross cell
Cell ids are simple: 0 (self), 1, 2, 3, 4 as the neighbors, clockwise, start at top
The cell read scope and write scope could be variable based

How is an entity component system of any use in this?
The entire memory area is fixed, except possibly the existence of
cells themselves which can be allocated on demand

- How would a cell access its neighbors? It should just have pointers to
  its current neighbors? Or at least entity ids?

  perhaps it can say: "read this index and amount onto instruction stack"
  and "write this amount from instruction stack to this index" as async
  commands.

- How is neighborhood maintained efficiently? Go through all
  cells and construct neighborhood maps for each of them in a hashtable?

  When we write async writing isn't too hard; this can be batched in order.

  When we read there's an async nature to it as well though. Could writing
  be done as part of input/output gates instead? Maybe the system can have
  multiple stacks and we can easily switch stack?

- What happens if we write to an empty neighbor? It's instantiated
  with Noop instructions (which cost nothing).

Async input/output gates:

- SelectOut - select output gate to use

- Out - push top of stack onto output gate

- OutFull - true if the output is full

- SelectIn - select input gate to use

- In - take input gate onto top of stack

- InEmpty - true if the input is empty

The writing north output gate:

- Out - index, value combination to write

- When consumed a single remaining number just remains on the queue.

How can we then process this efficiently? The ECS needs to be supplemented by a
neighbor map, which contains the write/read requests. There can be a write
request component. And a read request component. We then execute write
requests, which involves changing our own cell. Read request components
result in a modification of that read request by our current data.

Issue Write Request
Write Request stored with appropriate location

Process write request

Issue Read Request (which includes origin location)
Read request stored with appropriate location

Process read request, updating it and storing it with the appropriate
location it came from

cell, processors, environment

environment component has reference to other cell components
