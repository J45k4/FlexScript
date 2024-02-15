# FlexScript

## Functions

```
fn foo(a: u32, b: u32) -> u32 {
    return a + b
}

let r = foo(a = 1, b = 2)

// or

let r2 = foo {
    a: 1
    b: 3
}
```

## Structures

```
import std

struct Person {
    name: String
    age: u32
}
```

Anynomous objects

```
const obj = {
    name: "matti"
    age: 1234
}
```

## Operators

```
1 + 1 == 2 // Addition
1 - 1 == 0 // Substraction
2 * 2 == 4 // Multiplication
2 / 2 == 0 // Division
8 % 5 == 3 // Modulo operation
```

## Controlflow

### If

```
let a = 5
if a == 5 {

}
```

### For

```
const arr = [1, 2, 3, 4, 5]
for item in arr {

}
```

### While

```
let a = 0
while a < 5 {
    a++
}
```

## Types

### Pritimitive types

Primitive types are defined either with emulation or with compiler instruction.

```
@primitive("u32")
struct u32
```

```
struct bfloat {
    // Implementation
}
```

### Type alias

```
type Animal = {
    make_sound: fn () -> void
}

```

## Concurrency

### Require locks automatically

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

### Wait for coroutines

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

### Select one of the channels

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

### @pararell compiler instruction

Tell compiler that function migth be executed pararell. This could force depended code to handle synchronization.

```
fn thread(@pararell fn) {
    std.syscall(...)
}

```

### Iterators

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

### SIMD

```
import std.simmd

a = u32x4
b = u32x4
let c = a * b

```

## UI

```
Box {
    
}
```