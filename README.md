# micoos
arm1176 &amp; rust os


# MicOS

MicOS is the pet operating system targeting ARM processor and it
is currently in phase of pre-pre-alpha, as none of the core parts are
layed out, due the lack of time and knowledgea. It may never exit that phase
as well. 

## Abstract

The main idea is that we keep the bare minimum of stuff in the kernel -
currently only scheduling and message passing mechanism should do it.
Other than that, including drivers, file systems, memory management,
should never run in the supervisor mode.

The entire hardware should be abstracted in a single module/traits with
the clear interface (see `arm1176.rs` module).

The entire system is now very simple: after setting up the stack for the
modes, one of the processes is being pulled in by the scheduler module,
and executed until timer interrupt doesn't call ``scheduler::schedule_next``
again. System calls are implemented using swi interrupt, switching the mode
to the supervisor, and returning the value from the interrupt routine
with dropping to the user mode again.

In the near-far future, sometimes this year, there should be memory manager,
file system, ELF parser, and some device drivers in the phase that they
can be used for primitive loading the programs into the memory from the disk,
and executing them using exec call.

