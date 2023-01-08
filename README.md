## Run

```sh
cargo run -- init # create the database
cargo run -- add task "Read book"
cargo run -- list
```

## TODO

### Upcoming features:

- [x] Add task
- [x] List tasks
- [x] Mark tasks as done/undone
- [x] Store db in user's dir
- [ ] Support for ".env" files

### Dev tasks:
- [ ] Add unit tests
- [ ] Refactor database related stuff from `tasks.rs` to `db.rs`
