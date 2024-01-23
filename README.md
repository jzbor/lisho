# Lisho
A simple personal link shortener with no external dependencies in under 200 lines of Rust.

## Adding Link Mappings
Link mappings are maintained in a simple text file.
Entries consist of the short token and redirection URL separated by a whitespace.
It is also possible to add a redirection for the root path by adding a mapping with a leading whitespace.
Lines starting with a `#` are ignored, as are fields after the URL.
The filename must be passed to `lisho` on program start.

Example:
```
 https://github.com/jzbor/lisho
cb https://codeberg.org
gh https://github.com
gl https://gitlab.com
sh https://sr.ht
```
