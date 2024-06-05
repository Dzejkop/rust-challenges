# read-dir-bytes

This program accepts a path to a directory. Descends this directory recursively and finds all the files. The files are read one by one into memory.

While that is happening (with artificial slowness) a progress indicator should be updating - but it's not.

There's a single line change solution here.
