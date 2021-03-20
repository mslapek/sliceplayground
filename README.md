# Array duality demo

Demonstration of *array duality* (*owned* array vs *borrowed* array)
with [Go](https://golang.org/) and [Rust](https://www.rust-lang.org/).

## Photo album in Go

Directories `golangerr` and `golangok` contain photo album software demonstrating
array duality.

### What's inside?

`golangerr` demonstrates data corruption caused by not paying attention to array borrowing.

`golangok` is fixed version.

Notice, how `GetHolidayPhotos` is implemented in `golangerr/album.go` and `golangok/album.go`:

```
diff -U 15 golangerr/album.go golangok/album.go
```

### Running tests

You can run passing tests in `golangerr`:

```
cd golangerr
export GO111MODULE=off
go test
```

To run (failing) mutation test revealing the hidden bug:

```
cd golangerr
export GO111MODULE=off
MUTATION_TEST=1 go test
```

You can run all tests in fixed version `golangok`:

```
cd golangok
export GO111MODULE=off
go test
```

## Photo album in Rust

Directory `rust` contains ported photo album to *Rust*.

### What's inside?

Example shows how explicit distinction between `Vec<Photo>` and `&[Photo]`
made it impossible to reproduce the bug from first version.

Notice, how `get_holiday_photos` is implemented in `rust/src/album.rs`.

### Running tests

```
cd rust
cargo test
```

## Exercise

Optional exercise if you have time to spare.

Add to `GetHolidayPhotos` boolean arguments `june` and `december`. Return
photos only for requested months.

If only one month is requested, do *not* perform copy of any array,
this way avoiding unnecessary allocation on the heap.

Do exercise for `golangok` and `rust`.
