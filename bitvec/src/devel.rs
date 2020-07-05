/*! Utilities needed to develop `bitvec` itself.

This module contains required to perform generic programming in the `bitvec`
type system. These are not part of the SemVer public API, as they are only
required when interacting directly with the `bitvec` type system, and are not
needed to use its data structures directly.

This module is provided, under `feature = "devel"`, for the use of other crates
that wish to safely perform generic programming with `bitvec` region types.
!*/

#![allow(dead_code)]
#![cfg_attr(tarpaulin, skip)]

use crate::{
	access::BitAccess,
	index::BitMask,
	pointer::BitPtr,
	store::BitStore,
};

use core::ops::{
	Bound,
	Range,
	RangeBounds,
};

use wyz::pipe::Pipe;

/// Views a `BitStore` reference as its accessor.
#[inline(always)]
pub fn accessor<T>(x: &T) -> &T::Access
where T: BitStore {
	unsafe { &*(x as *const T as *const T::Access) }
}

/// Inserts an `::Alias` marker into a `BitMask`’s type parameter.
#[inline(always)]
pub fn alias_mask<T>(
	x: BitMask<T::Mem>,
) -> BitMask<<T::Alias as BitStore>::Mem>
where T: BitStore {
	unsafe { *(&x as *const _ as *const _) }
}

/// Inserts an `::Alias` marker into a `T::Mem` value’s type.
#[inline(always)]
pub fn alias_mem<T>(x: T::Mem) -> <T::Alias as BitStore>::Mem
where T: BitStore {
	unsafe { *(&x as *const _ as *const _) }
}

/// Loads through an aliased reference into an unmarked local.
#[inline(always)]
pub fn load_aliased_local<T>(x: &T::Alias) -> T::Mem
where T: BitStore {
	x.pipe(accessor::<T::Alias>)
		.pipe(BitAccess::load_value)
		.pipe(remove_alias::<T>)
}

/// Converts a mutable reference into its memory register type.
#[inline(always)]
pub fn mem_mut<T>(x: &mut T) -> &mut T::Mem
where T: BitStore {
	unsafe { &mut *(x as *mut _ as *mut _) }
}

/// Removes the `::Alias` marker from a register value’s type.
#[inline(always)]
pub fn remove_alias<T>(x: <<T as BitStore>::Alias as BitStore>::Mem) -> T::Mem
where T: BitStore {
	unsafe { *(&x as *const _ as *const _) }
}

/// Removes the `::Alias` marker from a `BitPtr`’s referent type.
#[inline(always)]
pub fn remove_bitptr_alias<T>(x: BitPtr<T::Alias>) -> BitPtr<T>
where T: BitStore {
	unsafe { *(&x as *const _ as *const _) }
}

/** Normalizes any range into a basic `Range`.

This unpacks any range type into an ordinary `Range`, returning the start and
exclusive end markers. If the start marker is not provided, it is assumed to be
zero; if the end marker is not provided, then it is assumed to be `end`.

The end marker, if provided, may be greater than `end`. This is not checked in
the function, and must be inspected by the caller.

# Type Parameters

- `R`: A range of some kind

# Parameters

- `bounds`: A range of some kind
- `end`: The value to use as the exclusive end, if the range does not have an
  end.

# Returns

`bounds` normalized to an ordinary `Range`, optionally clamped to `end`.
**/
#[inline]
pub fn normalize_range<R>(bounds: R, end: usize) -> Range<usize>
where R: RangeBounds<usize> {
	let min = match bounds.start_bound() {
		Bound::Included(&n) => n,
		Bound::Excluded(&n) => n + 1,
		Bound::Unbounded => 0,
	};
	let max = match bounds.end_bound() {
		Bound::Included(&n) => n + 1,
		Bound::Excluded(&n) => n,
		Bound::Unbounded => end,
	};
	min .. max
}

/** Asserts that a range satisfies bounds constraints.

This requires that the range start be not greater than the range end, and the
range end be not greater than the ending marker (if provided).

# Parameters

- `range`: The range to validate
- `end`: An optional maximal value that the range cannot exceed

# Panics

This panics if the range fails a requirement.
**/
#[inline]
pub fn assert_range(range: Range<usize>, end: impl Into<Option<usize>>) {
	if range.start > range.end {
		panic!(
			"Malformed range: `{} .. {}` must run from lower to higher",
			range.start, range.end
		);
	}
	if let Some(end) = end.into() {
		if range.end > end {
			panic!(
				"Range out of bounds: `{} .. {}` must not exceed `{}`",
				range.start, range.end, end
			);
		}
	}
}
