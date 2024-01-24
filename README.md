# Lisho
A simple personal link shortener with no external dependencies in under 200 lines of Rust.
The links are maintained as a simple text file on the host machine.

```
[jzbor@desktop-i5] ~ lisho mappings.txt
Listening on localhost:8080 (5 links)
Token requested: mars
Token requested: asdfasdf
...
```

## Adding Link Mappings
Lisho reads mappings from a simple text file.
Entries consist of the short token and redirection URL separated by a whitespace.
Lines starting with a `#` are ignored, as are fields after the URL.
It is also possible to add a redirection for the root path by adding a mapping with a leading whitespace.

Example:
```
cb https://codeberg.org
gh https://github.com
gl https://gitlab.com
sh https://sr.ht
```


## Static Files
There are some files that are compiled into `lisho` by default:
* `/`
* `/index.html`
* `/style.css`
* `404.html` for 404 errors

You can override these defaults by simply adding a mapping to your preferred pages, in which case `lisho` will redirect them as usual.
Similarly you can also set a favicon by redirecting it somewhere on the internet where your favicon is hosted.

```
# override index page
 https://github.com/lisho
index.html https://github.com/lisho

# add favicon
favicon.ico https://jzbor.de/favicon.ico
```

Of course this approach is rather limited, but `lisho`'s primary goal is simplicity.


## Convenient Alias
To make editing aliases on a remote machine easier you can add an alias in your shell config like so:
```sh
alias lisho-edit='ssh <hostname> -t <editor> <path>'
```

