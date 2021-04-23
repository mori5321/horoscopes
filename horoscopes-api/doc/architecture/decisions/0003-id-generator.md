# 3. ID Generator

Date: 2021-04-24

## Status

Accepted

## Context
I want to inject dependencies to generate UUID or ULID.
I think of two patterns of the interface of it.

#### Pattern1. TodoIDGenerator() -> todo::ID.
This returns Value Object(todo::ID).
Merit: And sometimes we can pass some argument to generate ID depending on entity type.
Cons: We have to implement id every time we add an entity.

#### Pattern2. IDGenerator() -> String
This returns just a String.
Pros: It is very simple.
Cons: Nothing for now. 

## Decision
Since Pattern1 looks too much for our project now, We decide to use Pattern2. 
If we need separated id generator, I think we can easily migrate from Pattern2 to Pattern1.

## Consequences

What becomes easier or more difficult to do and any risks introduced by the change that will need to be mitigated.
