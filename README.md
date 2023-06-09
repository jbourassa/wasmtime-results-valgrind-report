In this example, Valgrind finds a _unrecognised instruction at address_.
The culprit is a dynamic `Func` with many params (this example uses 200).
Credit for pinpointing the issue goes to @ianks.

To reproduce:

```
cargo build && valgrind target/debug/many-results
```

This is what I get:

```
==25125== Memcheck, a memory error detector
==25125== Copyright (C) 2002-2022, and GNU GPL'd, by Julian Seward et al.
==25125== Using Valgrind-3.21.0 and LibVEX; rerun with -h for copyright info
==25125== Command: target/debug/many-results
==25125==
ARM64 front end: load_store
disInstr(arm64): unhandled instruction 0xB830EBFF
disInstr(arm64): 1011'1000 0011'0000 1110'1011 1111'1111
==25125== valgrind: Unrecognised instruction at address 0x4f6fc2c.
==25125==    at 0x4F6FC2C: ???
==25125== Your program just tried to execute an instruction that Valgrind
==25125== did not recognise.  There are two possible reasons for this.
==25125== 1. Your program has a bug and erroneously jumped to a non-code
==25125==    location.  If you are running Memcheck and you just saw a
==25125==    warning about a bad jump, it's probably your program's fault.
==25125== 2. The instruction is legitimate but Valgrind doesn't handle it,
==25125==    i.e. it's Valgrind's fault.  If you think this is the case or
==25125==    you are not sure, please let us know and we'll try to fix it.
==25125== Either way, Valgrind will now raise a SIGILL signal which will
==25125== probably kill your program.
```
