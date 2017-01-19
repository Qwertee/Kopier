# Kopier
Simple command line text file copier

# Compiling
Simply run `cargo build` to compile and generate an executable at `target/debug/kopier`.

To run simply use the executable name followed by the path of the source file followed by the path of the
destination file.

For example (from within the project directory): `target/debug/kopier ~/test.txt /tmp/dest.txt`.

To call it from anywhere place it in your bin folder.

# NOTE 
Only works on .txt files with UTF-8 encoding. Other files will return an error.
