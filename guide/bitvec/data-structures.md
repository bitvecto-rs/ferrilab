# Data Structures

`bitvec`’s primary exports are four data structures: [`BitSlice`], [`BitArray`],
[`BitBox`], and [`BitVec`]. These correspond to the [`[bool]`][slice],
[`[bool; N]`][array], [`Box<[bool]>`][boxed], and [`Vec<bool>`] types in the
Rust standard libraries. Unlike the Rust types, the `bitvec` types are not
composable, and cannot be mixed with any other types, including pointers in the
standard library.

```admonish danger
You **cannot** produce `Box<BitSlice>`, `Rc<BitSlice>`, or `Arc<BitSlice>`;
attempts to do produce or use these may induce undefined behavior.
```

The `bitvec` types implement the APIs of their standard-library counterparts to
the fullest extent possible. The only missing feature is currently
`IndexMut<usize>`, preventing `collection[index] = bit;` assignment. This means
that, for users looking for compact `usize => bool` collections, substituting
types in your project codebase ought to be enough to make your project Just
Work™️.

<!--- Is there a reason there's an early return on each of these lines?--->

What makes `bitvec` unique is that `BitSlice` acts exactly like `[bool]`, and can only be used
through `&BitSlice` and `&mut BitSlice` references. No other Rust library has this capability.

Before we explore the data structures in more detail, there are three caveats I
must provide:

1. `bitvec` uses an opaque, custom, pointer representation for everything except
   `BitArray`. You may not inspect or modify this pointer. You may not use it as
   a type or value parameter in any other types or functions. You will break
   your program if you try.

   `bitvec` ensures that this pointer encoding does not fail the compiler and
   language requirements for reference correctness. The rules used to do so are
   internal to the crate, and will not be present outside it. `bitvec` pointers
   are perfectly safe to use, as long as you treat them as completely opaque and
   use *only* the interfaces provided.

   Standard-library smart pointers do not understand this encoding and would
   attempt to dereference the encoded pointer; this is why trying to use
   `BitSlice` with them is undefined.

1. These structures all have two type parameters, `<T: BitStore, O: BitOrder>`.
   These parameters are described in the next chapter. They govern the in-memory
   representation of data storage, but are not relevant to the general use of the
   handle types.

1. `bitvec` trades an increased memory space efficiency for decreased
   instruction size and speed efficiency. The compiler *may* optimize some of
   the costs away, but `bitvec` structures are not free to use. The “zero-cost”
   of the `bitvec` abstraction is that you cannot write a better bitset, and
   *not* that it is equal in runtime performance to an ordinary `bool` sequence.

Now that the disclaimers are over, we can talk about how the types actually
work.

[`BitArray`]: https://docs.rs/bitvec/latest/bitvec/array/struct.BitArray.html
[`BitBox`]: https://docs.rs/bitvec/latest/bitvec/boxed/struct.BitBox.html
[`BitSlice`]: https://docs.rs/bitvec/latest/bitvec/slice/struct.BitSlice.html
[`BitVec`]: https://docs.rs/bitvec/latest/bitvec/vec/struct.BitVec.html
[`Vec<bool>`]: https://doc.rust-lang.org/stable/alloc/vec/struct.Vec.html
[array]: https://doc.rust-lang.org/stable/std/primitive.array.html
[boxed]: https://doc.rust-lang.org/stable/alloc/boxed/struct.Box.html
[slice]: https://doc.rust-lang.org/stable/std/primitive.slice.html
