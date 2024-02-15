## SIMD

```
a = std.simmd(u32, 4)
b = std.simmd(u32, 4)


```

## Pritimitive types

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

## Type alias

```
type Animal = {
    make_sound: fn () -> void
}

```