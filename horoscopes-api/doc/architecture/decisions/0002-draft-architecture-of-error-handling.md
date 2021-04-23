# 2. Draft Architecture of Error Handling

Date: 2021-04-20

## Status

Accepted

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

## Analysis
We mainly have 4 layers which we have to occur error.
- Domains
- Usecases
- Filters
- Adapters(ex: repository, external client)

I think it is a good to define errors separately for these 4 layers like,
- Domain Error
- Usecase Error
- Filter Error
- Adapter Error

Each layer error should only converts into next level layer error.
```
Domain Error -> Usecase Error -> Filter Error
Adapter Error ->
```

We must be able to detect where the source or cause of an error, 
even if the error is like 'Filter Error including Usecase Error including Domain Error'.


## Decision
We define error layers as below.

```
Domain Error -> Usecase Error -> App Error -> (warp::rejection) -> (HTTP Error)
Adapter Error ->
```

### Domain Error
Domain Error is error about business rules.
ex) We cannot set Todo 'done' if title includes 'WIP' or 'DRAFT' text.

### Usecase Error
Usecase Error is a error about application rules.
ex) We cannot accept the Todo title of which length is less than 80 letters.
  - This belongs to application rule, because when we set this rule, we usually assume that database constraint or user interface constraint.

## Consequences
We can converts Domain specific error into HTTP Error.
And the type of error is statically analyzed to some extent.



