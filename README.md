In this example, Valgrind finds a _unrecognised instruction at address_.
The culprit is a dynamic `Func` with many params (this example uses 200).
Credit for pinpointing the issue goes to @ianks.

To reproduce:

```
cargo build && valgrind target/debug/wasmtime-many-results
```

This is what I get:

```
==167887== Memcheck, a memory error detector
==167887== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==167887== Using Valgrind-3.20.0 and LibVEX; rerun with -h for copyright info
==167887== Command: target/debug/wasmtime-many-results
==167887==
ARM64 front end: load_store
disInstr(arm64): unhandled instruction 0xB830EBFF
disInstr(arm64): 1011'1000 0011'0000 1110'1011 1111'1111
==167887== valgrind: Unrecognised instruction at address 0x4f6fc2c.
==167887==    at 0x4F6FC2C: ???
==167887== Your program just tried to execute an instruction that Valgrind
==167887== did not recognise.  There are two possible reasons for this.
==167887== 1. Your program has a bug and erroneously jumped to a non-code
==167887==    location.  If you are running Memcheck and you just saw a
==167887==    warning about a bad jump, it's probably your program's fault.
==167887== 2. The instruction is legitimate but Valgrind doesn't handle it,
==167887==    i.e. it's Valgrind's fault.  If you think this is the case or
==167887==    you are not sure, please let us know and we'll try to fix it.
==167887== Either way, Valgrind will now raise a SIGILL signal which will
==167887== probably kill your program.
```
