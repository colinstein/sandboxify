# Sandboxing Mac Applications

Sometimes it's nice to be able to run applications without letting them do "all
the things" on your system. Maybe you're writing some software and you don't
trust yourself not to have a burried `rm -rf ~/*` in there. Maybe it's a
program you don't trust from the internet, or maybe you're concerned about a
plug-in being compromised (e.g. a linter) and syphoning off data to the outside
world.

Mac OS comes with a built-in sandboxing system that lets you lock down applications
with a variety of rules. I wasn't really sure how that worked so I thought I'd make
a few demo programs and save some notes.

## Note
The examples make the following assumptions:
  1. Assuming this repository is cloned into your home directory
  2. You have built the relevant binaries
  3. `pwd` is the root of this repository
If not, you may need to edit the sandbox files to make things go.

# Using sandbox files

## Lock down all the things
This is going to do nothing because even running a program must be explicitly
enabled with the `(allow process-exec …)` option. This mostly exists to be a
very minimal starting point.

```
sandbox-exec -f noaccess.sb /path/to/bin
```

## Allow read-access to home directories
For this example I have a minimal rust program that will read a file passed
as a command line argument and write it to stdout like `cat`. You can build it
by running `rustc src/read_file.rc -o bin/read_file`.

This sandbox file allows access to /usr/lib because applications tend to need
a whole pile of system libraries to do anything useful. In principle you could
build a static stand-alone binary and lock things down, but I think that does
not make for a good example for setting things up.

```
sandbox-exec -f noaccess.sb bin/read_file README.md
```
If all goes well you should see the content of this file.

You can then try running the binary and having it read some other file outside
of home directories to see it get blocked and fail.

```
sandbox-exec -f noaccess.sb bin/read_file /etc/hosts
```
