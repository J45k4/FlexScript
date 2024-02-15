## Require locks automatically

```
import std

@threads(5) // works as marker for complier can use up to 5 threads for execution
fn main() {
    let var = 1 // should require lock

    fn foo() {
       var += 1 
    }

    std.co(foo())
    std.co(foo())
    std.co(foo())
}
```

## Wait for coroutines

```
import std

fn foo() {
    return 1
}

fn main() {
    let c1 = std.co(foo())
    let c2 = std.co(foo())
    let [v1, v2] = std.join([c1, c2])
}
```

## Select one of the channels

```

fn foo() {
    return 6
}

fn main() {
    let t1 = co(foo)
    let t2 = co(foo)

    select {
        r1 = t1 => {

        }
        r2 = t2 => {

        }
    }
}
```

## @pararell compiler instruction

Tell compiler that function migth be executed pararell. This could force depended code to handle synchronization.

```
fn thread(@pararell fn) {
    std.syscall(...)
}

```

## Iterators

```

fn foo() {
    return 5
}

fn main() {
    c1 = std.co(foo())
    c2 = std.co(foo())
    c3 = std.co(foo())

    let items = [c1, c2, c3]

    // Will wait coroutines in order they are defined
    for item in items {
        print(item)
    }

    // Will iterate co routines in order they complete
    for item in std.select(items) {
        print(item)
    }
}
```