## Run

```sh
cargo run -- init # create the database
cargo run -- add task "Read book"
cargo run -- list
cargo run -- done 1 # mark task with id 1 as done
cargo run -- undone 2 # mark task with id 2 as undone
```

## TODO

### Upcoming features:

- [x] Add task
- [x] List tasks
- [x] Mark tasks as done/undone
- [x] Store db in user's dir
- [ ] Support for ".env" files
- [ ] Document the CLI options

### Dev tasks:

- [ ] Add unit tests
- [ ] Refactor database related stuff from `tasks.rs` to `db.rs`
- [ ] Handle non-existing database
- [ ] Use [Diesel](https://diesel.rs/) as ORM
