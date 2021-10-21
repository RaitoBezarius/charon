struct Pair<T1, T2> {
    x: T1,
    y: T2,
}

enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

/// Sometimes, enumerations with one variant are not treated
/// the same way as the other variants (for example, downcasts
/// are not always introduced).
/// A downcast is the cast of an enum to a specific variant, like
/// in the left value of:
/// `((_0 as Right).0: T2) = move _1;`
enum One<T1> {
    One(T1),
}

/// Truely degenerate case
/// Instanciations of this are encoded as constant values by rust.
enum EmptyEnum {
    Empty,
}

/// Enumeration (several variants with no parameters)
/// Those are not encoded as constant values.
enum Enum {
    Variant1,
    Variant2,
}

/// Degenerate struct
/// Instanciations of this are encoded as constant values by rust.
struct EmptyStruct {}

enum Sum<T1, T2> {
    Left(T1),
    Right(T2),
}

fn test2() {
    let x: u32 = 23;
    let y: u32 = 44;
    let z = x + y;
    let p: Pair<u32, u32> = Pair { x: x, y: z };
    let s: Sum<u32, bool> = Sum::Right(true);
    let o: One<u64> = One::One(3);
    let e0 = EmptyEnum::Empty;
    let e1 = e0;
    let enum0 = Enum::Variant1;
}

fn get_max(x: u32, y: u32) -> u32 {
    if x >= y {
        x
    } else {
        y
    }
}

fn test3() {
    let x = get_max(4, 3);
    let y = get_max(10, 11);
    let z = x + y;
    assert!(z == 15);
}

/// Testing what happens with negation - in particular for overflows.
/// In debug mode, rust introduces an assertion before the negation.
fn test_neg(x: i32) -> i32 {
    -x
}

fn test_neg1() {
    let x: i32 = 3;
    let y = -x;
    assert!(y == -3);
}

fn refs_test() {
    let mut x = 0;
    let mut y = 1;
    let mut px = &mut x;
    let py = &mut y;
    let ppx = &mut px;
    *ppx = py;
    **ppx = 2;
    assert!(*px == 2);
    assert!(x == 0);
    assert!(*py == 2);
    assert!(y == 2);
}

/// Box creation
fn test_list1() {
    let l: List<i32> = List::Cons(0, Box::new(List::Nil));
}

/// Box deref
fn test_box1() {
    use std::ops::Deref;
    use std::ops::DerefMut;
    let mut b: Box<i32> = Box::new(0);
    let x = b.deref_mut();
    *x = 1;
    let x = b.deref();
    assert!(*x == 1);
}

fn copy_int(x: i32) -> i32 {
    x
}

// Just testing that shared loans are correctly handled
fn test_copy_int() {
    let x = 0;
    let px = &x;
    let y = copy_int(x);
    assert!(*px == y);
}

fn is_cons<T>(l: &List<T>) -> bool {
    match l {
        List::Cons(_, _) => true,
        List::Nil => false,
    }
}

fn test_is_cons() {
    let l: List<i32> = List::Cons(0, Box::new(List::Nil));

    assert!(is_cons(&l));
}

fn split_list<T>(l: List<T>) -> (T, List<T>) {
    match l {
        List::Cons(hd, tl) => (hd, *tl),
        _ => panic!(),
    }
}

fn test_split_list() {
    let l: List<i32> = List::Cons(0, Box::new(List::Nil));

    let (hd, tl) = split_list(l);
    assert!(hd == 0);
}

fn get_elem<'a, T>(b: bool, x: &'a mut T, y: &'a mut T) -> &'a mut T {
    if b {
        return x;
    } else {
        return y;
    }
}

fn get_elem_test() {
    let mut x = 0;
    let mut y = 0;
    let z = get_elem(true, &mut x, &mut y);
    *z = *z + 1;
    assert!(*z == 1);
    // drop(z)
    assert!(x == 1);
    assert!(y == 0);
}

fn id_mut_mut<'a, 'b, T>(x: &'a mut &'b mut T) -> &'a mut &'b mut T {
    x
}

fn id_mut_pair<'a, T>(x: &'a mut (&'a mut T, u32)) -> &'a mut (&'a mut T, u32) {
    x
}

fn id_mut_pair_test1() {
    let mut x: u32 = 0;
    let px = &mut x;
    let mut p = (px, 1);
    let pp0 = &mut p;
    let pp1 = id_mut_pair(pp0);
    let mut y = 2;
    *pp1 = (&mut y, 3);
}

fn id_mut_mut_pair<'a, T>(x: &'a mut &'a mut (&'a mut T, u32)) -> &'a mut &'a mut (&'a mut T, u32) {
    x
}

fn id_mut_mut_mut_same<'a, T>(x: &'a mut &'a mut &'a mut u32) -> &'a mut &'a mut &'a mut u32 {
    x
}

fn id_borrow1<'a, 'b, 'c>(_x: &'a mut &'b u32, _y: &'a &'a mut u32) {
    ()
}

/*struct WrapShared<'a, T> {
    x: &'a T,
}

impl<'a, T> WrapShared<'a, T> {
    /*    fn new(x: &'a T) -> WrapShared<'a, T> {
        WrapShared { x }
    }*/

    fn new2<'b, T2>(x: &'a T, y: &'b T2) -> (WrapShared<'a, T>, WrapShared<'b, T2>) {
        (WrapShared { x }, WrapShared { x: y })
    }
}

// TODO: what happens if I call WrapShared::new2? Are there lifetime parameters
// present in the function call?

fn wrap_shared_new2_test1() {
    let x: u32 = 0;
    let y: u32 = 1;
    let (px, py) = WrapShared::new2(&x, &y);
}

fn wrap_shared_new2_test2<'a, T>(x: &'a T) -> (WrapShared<'a, T>, WrapShared<'a, T>) {
    // The region variables are erased below, and we only see the early bound ones
    let (px, py) = WrapShared::<'a, T>::new2(x, x);
    (px, py)
}*/

/*fn f4<'a, 'b>(
    ppx: &'a mut &'b mut u32,
    ppy: &'a mut &'b mut u32,
) -> (&'a mut &'b mut u32, &'a mut &'b mut u32) {
    (ppx, ppy)
}*/

/*enum Enum1 {
    Case0(i32),
    Case1(i32),
    Case2(i32),
}*/

/*
fn enum1_get(x: Enum1) -> i32 {
    match x {
        Enum1::Case0(x) => x,
        Enum1::Case1(x) => x,
        Enum1::Case2(x) => x,
    }
}

fn test_match2() {
    let x = Enum1::Case1(0);
    let y = enum1_get(x);
    assert!(y == 0);
}*/

// TODO: switch test
// TODO: panic!

// TODO: &'a mut (&'a mut u32, u32)
// TODO: make an example with a match, to see how the discriminant is handled
// TODO: test binops, unops, etc.
// TODO: loops
// TODO: vectors
// TODO: arrays and slices
// TODO: intensive tests for two-phase borrows (https://rustc-dev-guide.rust-lang.org/borrow_check/two_phase_borrows.html)
// TODO: impl taking a self parameter (self, &self, &mut self)
// TODO: play with types and functions taking anonymous types and regions as parameters.
// TODO: functions with no parameters (we should extract them to: () -> ...)
// Test this on: top-level type declarations and functions and also impls.

fn main() {}