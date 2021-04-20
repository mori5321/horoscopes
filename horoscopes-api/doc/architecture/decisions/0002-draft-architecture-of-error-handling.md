# 2. Draft Architecture of Error Handling

Date: 2021-04-20

## Status

DRAFT

## Context
We need some error handling architecture.
For now, we just returns bool to express whether an usecase succeed or failed.

``` hoge_usecase.rs
fn run(&self) -> Output {
  // some code...
  match result {
    Some(v) => {
      Output { success: true }
    },
    None => {
      Output { success: false }
    }
  }
}
```

However, we need to control "what error ends withs what kind of response".
For example, if record not found on repository layer, we usually want to respond with Client Error 404 - Not found.
In other case, if validation error occurred on domain layer, we usually want to respond with Client Error 422 - Unprocessable Entity.

Therefore, we must satisfy 3 constraints about our error handling.
- We can convert error to some HTTP Response & Status Code.
- We can detect where the error occurred. (logging, separate layer)

## Decision
WIP...

## Consequences
WIP...
