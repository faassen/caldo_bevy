# Kind

various chemicals
and other general world entities

# Id

creature id

Do we want varieties of instructions that look for id versus kind?

# Action

## rotate (lr --)

Rotate impulse

## thrust (side strength --)

## stick (side kind --)

## unstick (side kind --)

## attack (side --)

## wall (side --)

Thrust in direction

## smell (kind -- strength)

Smell for kind, give back strength of smell.

This is determined by taking a bounding box/circle around the creature, adding
up all the kind. How to normalize?

## look (side -- distance kind)

Look in direction.

## touch (side -- kind)

Determine what's attached to side, if anything.

# Creation

## cell (side --)

Create new cell on side.

## processor (side gene_id --)

Create new processor on gene on side.

## self_processor (gene_id --)

Create new processor on gene.

## processor_stop

Processor stops & disappears.

## read (side gene_id index -- value)

## write (side gene_id index value -- )

## self_read (gene_id index -- value)

## self_write (gene_id index value -- )

# Exchange

## transport_in (side kind --)

## transport_out (side kind --)

## say (side value -- )

## hear (side -- value)

## inject (side kind --)

Produce chemical. If there is a stuck creature on the side, and permissions
allow it, chemical will go into it. If not allowed, operation does nothing.

If there is nothing on side, chemicals are put into environment as a little
blob?

## extract (side kind --)

# Sensory

# Permission

# gene_read (side --)

Allow gene read by neighbor. Automatic if wall < 50 %.

# gene_write (side --)

Allow gene write by neighbor. Automatic if wall > 25%.

(is there some sneaky way to do this otherwise? perhaps a wall weakening
chemical)

# chem_injection (side --)

Allow chemical injection by neighbor. AUtomatic if wall < 75%.

# chem_extraction (side --)

Allow chemical extraction by neighbor. Automatic if wall < 25%.
