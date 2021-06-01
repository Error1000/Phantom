Phantom
-------

A simple RV32E cpu, that is spec compliant( i think i haven't really tun tests yet ) except for:
 - it supports only the user isa
 - no interrupts
 - no support for missaligned addresses ( well actually you can do a jalr with missaligned address as long as they add up to an aligned address )
 - memory ordering instruction is not implemented yet

NOTE: The AUIPC instruction will
      just chop off the high 2 bits
      to convert the native 4-byte address
      which can cover 16 gb to a byte-oriented
      address, which means it won't work correctly
      outside the first 4 gb, LUI will be fine tho.

NOTE: Right now, an exception does not get triigered
      on missaligned address, and they will just be silently truncated.
      However i plan to just HLT the cpu in the future.

NOTE: Should do a cleanup of the circuits, simplifying control lines
      and unify naming, because rn it's kinda all over the place 
      because control lines, the units of control in this cpu,
      have varying degrees of granularity, maybe one might do
      only one thing like the OUT ones will generally only
      trigger a tri-state gate, or a lot of changes like AAU_SWTCHMOD
      which will basically completly chage the behavieour of the AAU
      in several ways.
      TL;DR The circuits need a bit of cleanup and the naming 
      of control lines should be more unified.
      
Status:
      Thankfully right now, it's still quite managable and i'm pretty
      close to finishing the base rv32e user ISA, so i think i'll
      finish that and then do a cleanup, then i'll see if i want
      to add more, but i definetly want to get some software
      pretty soon (TM) tho, so i might only do the base use isa for
      this project.
