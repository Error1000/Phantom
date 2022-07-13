Phantom
-------

A simple RV32E cpu, that is spec compliant( i think i haven't really run tests yet ) except for:
 - it supports only the user isa
 - no interrupts or exceptions yet
 - no support for misaligned addresses ( well actually you can do a jalr with misaligned address as long as they add up to an aligned address )

NOTE: The AUIPC instruction will
      just chop off the high 2 bits
      to convert the native 4-byte address
      which can cover 16 gb to a byte-oriented
      address, which means it won't work correctly
      outside the first 4 gb, LUI will be fine tho.

NOTE: Right now, an exception does not get triggered
      on misaligned address on jmp/branch, and they will just be silently truncated ( first 2 bits get set to 0,
      to make number divisable by 4).
      However i plan to just HLT the cpu in the future.

NOTE: Should do a cleanup of the circuits, simplifying control lines
      and unify naming, because rn it's kinda all over the place 
      because control lines, the units of control in this cpu,
      have varying degrees of granularity, maybe one might do
      only one thing like the OUT ones will generally only
      trigger a tri-state gate, or a lot of changes like AAU_SWTCHMOD
      which will basically completely change the behavior of the AAU
      in several ways.
      TL;DR The circuits need a bit of cleanup and the naming 
      of control lines should be more unified.

NOTE:  Should probs add special instruction to set the 2 high bits of ram address that cannot be set normally.
       Could probablly use that for nice out of the way mmaped-io.
       
Status:
      Stalled, probablly not gonna spend more time on this project.
