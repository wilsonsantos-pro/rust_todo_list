## Run

```sh
cargo run -- init # create the database
cargo run -- add task "Read book"
cargo run -- list
cargo run -- done 1 # mark task with id 1 as done
cargo run -- undone 2 # mark task with id 2 as undone
```

## Available features

- [x] Add task
- [x] List tasks
- [x] Mark tasks as done/undone
- [x] Delete task (0.2.0)

## Upcoming features

- [ ] Sort task by done, created
- [ ] Bulk operations:
  - [ ] Delete list: `delete 1,2,3`
  - [ ] Delete range: `delete 1..3`
  - [ ] Mark done list: `done 1,2,3`
  - [ ] Mark done range: `done 1..3`
  - [ ] Mark undone list: `undone 1,2,3`
  - [ ] Mark undone range: `undone 1..3`
- [ ] Edit title
- [ ] Add due date
- [ ] Sort by due date
- [ ] Support for ".env" files
- [ ] Document the CLI options

## TODO: Dev tasks

- [x] Store db in user's dir
- [x] Refactor database related stuff from `lib.rs` to `crud.rs`
- [ ] Add unit tests
- [ ] Handle non-existing database
- [ ] Use [Diesel](https://diesel.rs/) as ORM
