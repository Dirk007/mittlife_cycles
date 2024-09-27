<img src="assets/logo.png" width="200" align="left" style="margin: 4px; align: left;" /> 

# mittlife_cycles

**the mittwald extension signature verifier.**

<br>
<hr>

## Purpose

mittlife_cycles is a crate to verify mittwald marketplace signatures for your extension backends written in `rust`.

## Sample

The sample backend implemented here does NOTHING beside verifying the signature. 

To run the simple example using poem use
```bash
cargo run --example simple
```

## Features

mittlife_cycles comes with feature flags to add built-in header-support different web-frameworks:
- poemheader - For [poem](https://github.com/poem-web/poem) - `default`
- actixheader - for [actix](https://actix.rs/)


### License

[MIT](LICENSE)

Cheers
