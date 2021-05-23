Phantom
-------

A simple RV32E cpu.

NOTE: The AUIPC instruction will
      just chop off the high 2 bits
      to convert the native 4-byte address
      which can cover 16 gb to a byte-oriented
      address, which means it won't work correctly
      outside the first 4 gb, LUI will be fine tho.
