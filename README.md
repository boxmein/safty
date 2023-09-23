# safty

```
⠀⠀⠀⠀⠀⢀⠤⠐⠒⠀⠀⠀⠒⠒⠤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⡠⠊⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⢄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⡔⠁⠀⠀⠀⠀⠀⢰⠁⠀⠀⠀⠀⠀⠀⠈⠆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⢰⠀⠀⠀⠀⠀⠀⠀⣾⠀⠀⠔⠒⠢⠀⠀⠀⢼⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⡆⠀⠀⠀⠀⠀⠀⠀⠸⣆⠀⠀⠙⠀⠀⠠⠐⠚⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠇⠀⠀⠀⠀⠀⠀⠀⠀⢻⠀⠀⠀⠀⠀⠀⡄⢠⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡀⠀⠀
⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⣀⣀⡠⡌⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⢄⣲⣬⣶⣿⣿⡇⡇⠀
⠀⠀⠆⠀⠀⠀⠀⠀⠀⠀⠘⡆⠀⠀⢀⣀⡀⢠⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⢴⣾⣶⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀
⠀⠀⢸⠀⠀⠀⠀⠠⢄⠀⠀⢣⠀⠀⠑⠒⠂⡌⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⢿⣿⣿⣿⣿⣿⣿⣿⡇⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠑⠤⡀⠑⠀⠀⠀⡘⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⣡⣿⣿⣿⣿⣿⣿⣿⣇⠀
⠀⠀⢀⡄⠀⠀⠀⠀⠀⠀⠀⠈⢑⠖⠒⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐⣴⣿⣿⣿⡟⠁⠈⠛⠿⣿⠀
⠀⣰⣿⣿⣄⠀⠀⠀⠀⠀⠀⠀⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⢈⣾⣿⣿⣿⠏⠀⠀⠀⠀⠀⠈⠀
⠈⣿⣿⣿⣿⣷⡤⣀⡀⠀⠀⢀⠎⣦⣄⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣢⣿⣿⣿⡿⠃⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠘⣿⣿⣿⣿⣿⣄⠈⢒⣤⡎⠀⢸⣿⣿⣿⣷⣶⣤⣄⣀⠀⠀⠀⢠⣽⣿⠿⠿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠹⣿⣿⣿⣿⣿⣾⠛⠉⣿⣦⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⡗⣰⣿⣿⣿⠀⠀⣿⠀⠀

              safty
```

safty is a Rust application project to quickly build a tailor-made WAF reverse
proxy.

:crab: :crab: :crab:

## Usage

* Clone the project
* Modify the WAF ruleset in [main.rs:91](https://github.com/boxmein/safty/blob/master/applications/saftyreverseproxy/src/main.rs#L91)
* Rebuild and run

Env variables to use:

```shell
# set log level
RUST_LOG=trace

# the application will accept incoming HTTP requests, and point them to the 
# protocol + host + port
# indicated in this URL
DOWNSTREAM_URI=http://foo.bar:8080/
```

## WAF Rules

Rules are all written into the codebase, no config file format exists right now.

Rules are evaluated from the top down, and combinators are available to 
chain together complex rules.

### Examples

Examples are as follows:

#### Path filtering

This rule blocks access to the folder /admin.

Read like:

* All of the above conditions must match:
  1. If path is /admin: drop, else take the next rule
  2. If method is POST and URL matches ^/photos, drop. Else, take the next rule.
  3. allow

```rust
AllPassFilter::new(vec![
  Box::new(ScopeRegexPathFilter::new("^/admin", DropFilter::new())),
  Box::new(ScopeRegexMethodPathFilter::new("POST", "^/photos", DropFilter::new())),
  Box::new(PassFilter::new())
])
```


#### Single quotes detector

The filter drops requests that contain a single quote in the body, and passes
other requests.

Read like:

* All of the above conditions must match:
  1. If single quote in request: drop
  2. allow

```rust
AllPassFilter::new(vec![
  Box::new(SingleQuotesFilter::new()),
  Box::new(PassFilter::new())
])
```

### Filter API

A filter must return Pass, Drop or NotMyProblem given a HTTP request. If the filter
returns Pass, the request is passed. If the filter returns Drop, the request is
dropped (ignored). If the filter returns NotMyProblem, the next filter is consulted. 

#### `PassFilter`

Returns Pass for all requests. Useful for combining with ScopeRegex etc.

#### `DropFilter`

Returns Drop for all requests. Useful for combining with ScopeRegex etc.
 
#### `AllPassFilter` combinator

Accepts a Vec of rules. Evaluates them in order. The first rule that returns
`Pass` or `Drop` wins.

```rust
AllPassFilter::new(vec![
  ... rules,
  // probably you want Pass or Drop as the last rule
  Box::new(PassFilter::new())
])
```

#### `ScopeRegexPathFilter` combinator

Accepts a single rule. Applies the inner rule only if the request's path matches
a given regex.

```rust
ScopeRegexPathFilter::new("^/admin", PassFilter::new())
```


#### `ScopeRegexMethodPathFilter` combinator

Accepts a single rule. Applies the inner rule only if the request's path matches
a given regex and the method matches the given method.

```rust
use hyper::http::Method;

ScopeRegexMethodPathFilter::new(Method::POST, "^/admin", PassFilter::new())
```

#### `InvertFilter` combinator

Accepts a single rule. Inverts the inner rule's decision (Drop => Pass, Pass => Drop)

```rust
InvertFilter::new(PassFilter::new()) // same as DropFilter
```

#### `SingleQuotesFilter`

Drops if the HTTP request body contains a single quote (byte 0x27)

```rust
SingleQuotesFilter::new()
```
