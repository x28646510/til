# TIL Actix

## Done

- `cargo install diesel_cli --no-default-features --features "postgres"`

```graphql
mutation createUser($userInput: UserInput = {name: "ko", email: "x28646510@gmail.com"}) {
  createUser(user: $userInput) {
    id
  }
}
```
