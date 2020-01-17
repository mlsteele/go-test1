# go-test1

A tool that runs a single go test. If the test is not the current directory but instead in a subdirectory, go-test1 will seek it out and run it.

```
go-test1 1.0
Go test runner

USAGE:
    go-test1 [OPTIONS] <NAME>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l <LOG>        File in which to save the log. Defaults to /tmp/test.log

ARGS:
    <NAME>    Name of the go test. With or without the initial 'Test'.
```


```
$ go-test1 TestFormatAmount
Test name: TestFormatAmount
Command: go test -v -run "^TestFormatAmount$"
found test in file: ./go/stellar/stellar_test.go
=== RUN   TestFormatAmount
--- PASS: TestFormatAmount (0.02s)
...
PASS
ok  	github.com/keybase/client/go/stellar	0.335s
```
