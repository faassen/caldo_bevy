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
