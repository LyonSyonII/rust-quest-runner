# Rust Quest Runner
Runner for the [Rust Quest book](https://garriga.dev/rust-quest) code snippets.

Built as a faster alternative to the [Rust Playground](https://play.rust-lang.org/) by running inside a single container.

Check it out at [https://garriga.dev/rust-quest/]!

## Installation
Just `docker run lyonsyonii/rust-quest-runner` and send your requests to `{IP_ADDRESS}:3030/evaluate.json`.

## Usage
The runner accepts requests in the following format:
```json
{
    "code": "fn main() { println!(\"Hello, World!\") } "
}
```

And replies with:
```json
{
    "ok": {
        "stdout": "Hello, World!",
        "stderr": "Compiler output"
    }
}
```
If the request went well.

If some error was encountered, it will send one of the following replies:
- STD,
- CORE,
- EXTERN_C,
- UNSAFE,
- TEMP_DIR,
- INPUT_FILE_CREATE,
- INPUT_FILE_OPEN,
- INPUT_FILE_WRITE,
- BUILD,
- COMPILER: "Compiler error message",
- TIMEOUT,
- EXECUTION: "Execution error message",

### Environment variables
| Name              | Default | Description                                                                                                 |
| ----------------- | ------- | ----------------------------------------------------------------------------------------------------------- |
| PORT              | 3030    | Port to listen on.                                                                                          |
| AUTH              | ""      | Customizable authorization token, sets the "Authorization" header.                                          |
| ORIGINS_WHITELIST | ""      | Comma separated list of allowed origins, sets `cors`.                                                       |
| SEMAPHORE_PERMITS | 5       | How many requests to allow at the same time, to avoid overload.                                             |
| SEMAPHORE_WAIT    | 500ms   | Time in milliseconds that a request will wait when the number of requests is larger than SEMAPHORE_PERMITS. |
| KILL_TIMEOUT      | 500ms   | Time in milliseconds that a request will be killed if it takes longer than.                                 |

## Safety
The runner does not allow any request with:
- Unsafe code (and the `unsafe` keyword)
- `extern "C"`
- `::std`
- `::core`

Additionally, it redeclares the `std` and `core` modules to only allow certain modules to be used.  
To see all the allowed modules, see the [`custom std`](project-template/custom-std/src/lib.rs).  
`custom-core` does not export anything.

This should eliminate all file and network access from the executed code.  
If you find any vulnerabilities, please [open an issue](https://github.com/lyonsyonii/rust-quest-runner/issues).

## License
[MIT](https://www.tldrlegal.com/license/mit-license)