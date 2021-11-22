//! Shamelessly Mimicking and Subsequently Butchering the Technical Interview
//! 
//! (This was written with very little planning)

/* 
As you enter the room, you inhale deeply. The air is dry, flaky with the taste of drywall. A worrisome place, akin to a sapling 
strung on a pendant. You make a mental note not to trust the cranes for directions. 

The woman behind the worn oak desk gestures you to sit. The name tag on her chest tells you she is Christina. You blink a warm 
smile. To name a heart is to know it; to wear its name is to wield it. Your own name sprouted many summers after its seedling 
burrowed into soil. 

"Hi!" she leads. "Nice to meet you. Please get comfortable! We're going to go through a programming puzzle step by step. You know, 
to see how you solve problems." Her scurrying words remind you of a white wagtail. You wonder whether she's actually sitting or 
simply flitting above her chair fast enough to maintain the illusion.

"Here's the problem. You have two strings, a needle and a haystack. You need to find the first index where the pattern is found 
inside the base. You should write a function that takes those two strings as input and returns that index, or something else if 
the needle isn't inside the haystack."

Ah, so the blind mother feeds her child in the nest by night. She finds her young by touch. Poor soul. Under your breath, you utter 
a brief prayer for health. You look back at Christina inquiringly.

"You can write this in any language. Our startup mostly uses Python and C, though. Oh, and a little bit of Rust."

"That will work well," you nod. This place might have some life in it after all. You whisper open a terminal, click your right 
index finger into your left palm and begin the dance: `cargo new --lib caw`

You are eager to begin.
*/
#![allow(incomplete_features)]
#![allow(private_in_public)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![recursion_limit = "512"]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]
#![feature(const_raw_ptr_deref)]
#![feature(const_type_name)]
#![feature(specialization)]
/*
"Uh, what are these for? Are you sure we need that? These are nightly features..." 

You smile the traces of an apology, but do not delete a line. Every keystroke is as it should be.

"The mother feeds her young in the night. It is only fair for us to also work in Night."

Christina pretends to understand. You can feel her brow furrowing in concern. Maybe her tail is darting about less frequently. 
You cannot tell.

"We will need a list," you reassure her. To more comforting definitions.
*/
use std::marker::PhantomData as Marker;

struct Nil;
struct Cons<A, B> (Marker<A>, Marker<B>) where B: List;

trait List {}
impl List for Nil {}
impl<A, B> List for Cons<A, B> where B: List {}

macro_rules! tfn {
    ($($x:tt)*) => {
        trait $($x)* {type Output;}
    };
}
macro_rules! tret {
    ($x:ty) => {
        type Output = $x;
    };
}
macro_rules! dtret {
    ($x:ty) => {
        default type Output = $x;
    };
}
macro_rules! tcall {
    ($x:ident $(<$y:tt>)?, $($z:tt)*) => {
        <$($z)* as $x$(<$y>)?>::Output
    };
}

tfn!(Head);
impl Head for Nil {
    tret!(Nil);
}
impl<A, B> Head for Cons<A, B> where B: List {
    tret!(A);
}
/*
"A... linked list? Surely there are more efficient collections in Rust? And besides, what's with all these traits?"

"Worry not, Christina. It will be as efficient as a cockatoo solving a puzzle."

You remember when you were but a hatchling. You sat perched in moonlight, challenging the stars to count themselves. Your mind 
was streaked with white ink, and soon the nature flushed over you. Emboldened, you foolishly challenged the empty space between 
the stars.

It was the first lesson of humility you learned. Channel that now; count with heed.
*/
struct Z;
struct S<T> (Marker<T>) where T: Num;

trait Num {}
impl Num for Z {}
impl<T> Num for S<T> where T: Num {}
/*
On second thought, you decide to let Christina easy this time. Yield not to the hubris of flight, you think to yourself as you 
beckon the elders, the primitives.
*/
pub struct Int<const N: usize>;

macro_rules! valid {
    ($x:expr) => {
        [(); {$x as usize}]
    };
}

tfn!(NextInt);
impl<const N: usize> NextInt for Int<N> where valid!(N + 1): {
    tret!(Int<{N + 1}>);
}
tfn!(ToInt);
impl ToInt for Z {
    tret!(Int<0>);
}
impl<N> ToInt for S<N> where N: Num + ToInt, tcall!(ToInt, N): NextInt {
    tret!(tcall!(NextInt, tcall!(ToInt, N)));
}

tfn!(ToNum);
macro_rules! failure { ($($x:tt)*) => {}; }
failure!{
    impl ToNum for Int<0> {
        tret!(Z);
    }
    impl<const N: usize> ToNum for Int<N> where valid!(N - 1): {
        dtret!(S<tcall!(ToNum, Int<{N - 1}>)>);
    }
    // error[E0080]: evaluation of `<Int<0_usize> as ToNum>::{constant#0}` failed
    // --> src/lib.rs:113:52
    //     |
    // 113 | impl<const N: usize> ToNum for Int<N> where valid!(N - 1): {
    //     |                                                    ^^^^^ attempt to compute `0_usize - 1_usize`, which would overflow
    // 
    // For more information about this error, try `rustc --explain E0080`.
}
/*
You look at the complaint. This isn't right... What went wrong?

You come to a realization. Ferris is strict on how they choose to specialize.

Maybe altering the rhymes will appease them.
*/
failure!{
    impl<const N: usize> ToNum for Int<N> {
        dtret!(Z);
    }
    impl<const N: usize> ToNum for Int<N> where valid!(N - 1):, Int<{N - 1}>: ToNum {
        tret!(S<tcall!(ToNum, Int<{N - 1}>)>);
    }
    // error[E0275]: overflow evaluating the requirement `Int<{N - 1}>: ToNum`
    //   |
    //   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "512"]` attribute to your crate (`caw`)
    //   = note: required because of the requirements on the impl of `ToNum` for `Int<{N - 1}>`
    //   = note: 255 redundant requirements hidden
    //   = note: required because of the requirements on the impl of `ToNum` for `Int<{N - 1}>`
    // 
    // For more information about this error, try `rustc --explain E0275`.
}
/*
The frost around your eyes begins to thaw. Surely Ferris doesn't need to check every number individually?

Christina is eyeing you with a sharp look. You guess there will be about two minutes until she realizes what you're doing with the 
type system and stops you. 

"We will want to build up some foundations first," you try to delay. "It's more rigorous that way!"

Macros have never failed you before. Let that be a blessing.
*/
macro_rules! church {
    () => {
        Z
    };
    ($x:tt $($y:tt)*) => {
        S<church!($($y)*)>
    }
}
macro_rules! primitive {
    () => {
        0
    };
    ($x:tt $($y:tt)*) => {
        1 + primitive!($($y)*)
    }
}
macro_rules! impl_iota {
    () => {
        impl ToNum for Int<0> {
            tret!(Z);
        }
    };
    ($x:tt $($y:tt)*) => {
        impl ToNum for Int<{1 + primitive!($($y)*)}> {
            tret!(S<church!($($y)*)>);
        }
        impl_iota!($($y)*);
    };
}
macro_rules! impl_exp {
    (: $x:literal $($y:literal)*) => {
        impl_iota!($($y)*);
    };
    ($x:literal $($y:literal)* : $($z:literal)*) => {
        impl_exp!($($y)* : $($z)* $($z)*);
    };
}
impl_exp!(0 0 0 0 0 0 0 0 : 1);
/*
That will work. You averted the crisis. Thank you Ferris for your concern.

Now, some alchemy...
*/
failure!{
    use std::intrinsics::transmute;
    
    struct String<const S: &'static str>;
    
    tfn!(ToParts);
    type Parts = (usize, usize);
    const fn raw_parts(s: &'static str) -> Parts {
        unsafe {transmute(s)}
    }
    const fn l(p: Parts) -> usize {p.0}
    const fn r(p: Parts) -> usize {p.1}
    impl<const S: &'static str> ToParts for String<{S}> where valid!(l(raw_parts(S))):, valid!(r(raw_parts(S))): {
        tret!((Int<{l(raw_parts(S))}>, Int<{r(raw_parts(S))}>));
    }
    type Haystack = tcall!(ToParts, String<"Hello, world!">);
    const HAYSTACK: &'static str = std::any::type_name::<Hay>();
    // error[E0080]: it is undefined behavior to use this value
    //    --> src/lib.rs:92:14
    //     |
    // 92  |         [(); {$x as usize}]
    //     |              ^^^^^^^^^^^^^ type validation failed: encountered pointer to alloc184, but expected initialized plain (non-pointer) bytes
    // ...
    // 189 | impl<const S: &'static str> ToParts for String<{S}> where valid!(l(raw_parts(S))):, valid!(r(raw_parts(S))): {
    //     |                                                           ----------------------- in this macro invocation
    //     |
    //     = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
    //     = note: the raw bytes of the constant (size: 8, align: 8) {
    //                 ╾──────alloc184───────╼                         │ ╾──────╼
    //             }
    //     = note: this error originates in the macro `valid` (in Nightly builds, run with -Z macro-backtrace for more info)
    //
    // For more information about this error, try `rustc --explain E0080`.
}
/*
Oh no. You have just invoked compile-time undefined behavior.

What now? Christina is eyeing you. 

"Uh oh! That looks scary. Maybe you should stick to safe code?"

You agree with Christina this time and gladly take the advice, even though you have already decided on what to write next.

"Yes, let's stick to safety."
*/
const fn idx(s: &'static str, i: usize) -> usize {
    s.as_bytes()[i] as usize
}
macro_rules! list {
    ($s:literal) => {
        Nil
    };
    ($s:literal $x:literal $($y:literal)*) => {
        Cons<tcall!(ToNum, Int<{idx($s, primitive!($($y)*))}>), list!($s $($y)*)>
    }
}
macro_rules! string {
    (type $n:ident = $s:literal : $($l:literal)* ;) => {
        type $n = list!($s $($l)*);
    };
}
string!{
    type Mother = "Hello, world!" : 1 1 1 1 1 1 1 1 1 1 1 1 1;
}
string!{
    type Child = "world" : 1 1 1 1 1;
}
/*
"Now what on earth is this?"

"A necessity," you respond. "Ferris is not being gentle with us today."

"Wait, what do all these macros expand to?"

"About... 60 thousand lines of trait implementations. It will make the solution tidier!" you add on hurriedly. As you do so, you 
try to comfort Ferris. You promise not to make them recur more than they want.

You continue. "And also I think it's a little funny," you smile with honesty.

"Well I won't deny that," Christina retorts. "I see you've built a lot of groundwork already. Where is it all going?"

"A little patience, my starling. We first need a larger nest."

"Did you just call me a starling?"

"Yes, is that alright?" You speak idly while thinking of what to implement next.

"Sure ???" Christina seems a little caught off guard. Maybe the word has special meaning for her. You won't prod, though; you have 
a crab to appease.
*/
struct True;
struct False;

trait Bool {}
impl Bool for True {}
impl Bool for False {}

tfn!(Not);
impl Not for True {
    tret!(False);
}
impl Not for False {
    tret!(True);
}

macro_rules! op {
    ($n:tt $(($l:ty, $r:ty) => $o:ty),*) => {
        tfn!($n);
        $(
            impl $n for ($l, $r) {
                tret!($o);
            }
        )*
    };
}

op!{And 
    (True, True) => True,
    (True, False) => False,
    (False, True) => False,
    (False, False) => False
}

op!{Or 
    (True, True) => True,
    (True, False) => True,
    (False, True) => True,
    (False, False) => False
}

tfn!(Eq);
impl Eq for (Z, Z) {
    tret!(True);
}
impl<N> Eq for (Z, S<N>) where N: Num {
    tret!(False);
}
impl<N> Eq for (S<N>, Z) where N: Num {
    tret!(False);
}
impl<N, M> Eq for (S<N>, S<M>) where N: Num, M: Num, (N, M): Eq {
    tret!(tcall!(Eq, (N, M)));
}
/*
"It is important for us to know truth from falsehood," you explain. "Far too often the young strays far from the nest in pursuit of a lie."

Christina blinks. You get the impression that she's given you free reign, and just wants to see what you'll do with it. You happily oblige.
*/
failure!{
    tfn!(Fn<T>);
    
    tfn!(Map);
    impl<F> Map for (F, Nil) {
        tret!(Nil);
    }
    impl<F, T, U> Map for (F, Cons<T, U>) where F: Fn<T>, U: List, (F, U): Map {
        tret!(Cons<tcall!(Fn<T>, F), tcall!(Map, (F, U))>);
    }
    // error[E0275]: overflow evaluating the requirement `(_, Nil): Map`
    //   |
    //   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "1024"]` attribute to your crate (`caw`)
    //   = note: required because of the requirements on the impl of `Map` for `(_, Cons<_, Nil>)`
    //   = note: 511 redundant requirements hidden
    //   = note: required because of the requirements on the impl of `Map` for `(_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, 
    //   Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Cons<_, Nil>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    //   >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    //   >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>)`
    //
    // For more information about this error, try `rustc --explain E0275`.
}
/*
You blink a triage spell. Could this really be happening?

This... this is a problem. Is Ferris really this eager?

You remember when you were a nestling. The desire to aid the ill-fated. The urge to mother, to feed my own hatchlings. Is this...

"Ooh, that's a nasty looking error. Do you need some help?" Christina interrupts your memories. "We can walk through the requirements 
step by step! Okay, so the haystack and needle are both strings. What we want now is a way to match substrings, and-"

"Thank you Christina, I think I can..." Your mind scours frantically for a solution. You can't do anything without lazily evaluated 
recursive trait bounds.

"... Maybe I'll use more macros." But no, that wouldn't work either! Macrons are untyped.

Perhaps... Perhaps you have to leave the types behind. It will be forever in the file, the relics of an abandoned nest. As much as it 
pains you, it'll have to be done.

You refuse to do runtime computation. Ferris deserves better.
*/
struct Input<const A: &'static str, const B: &'static str>;

tfn!(Solution);
const fn solution(a: &'static str, b: &'static str) -> usize { solve(a, b, 0) }
const fn solve(a: &'static str, b: &'static str, i: usize) -> usize {
    if i == a.len() - b.len() {
        !0usize
    }
    else if verify(a, b, i, 0) {
        i
    }
    else {
        solve(a, b, i + 1)
    }
}
const fn verify(a: &'static str, b: &'static str, i: usize, j: usize) -> bool {
    if j == b.len() {
        true
    }
    else {
        a.as_bytes()[i + j] == b.as_bytes()[j] && verify(a, b, i, j + 1)
    }
}
impl<const A: &'static str, const B: &'static str> Solution for Input<A, B> where valid!(solution(A, B)): {
    tret!(Int<{solution(A, B)}>);
}
pub type Output<const A: &'static str, const B: &'static str> = tcall!(Solution, Input<A, B>);
/*
"Squawk!" you exclaim. "It's finished."

"Okay! Where's the entry point? The `solution` function isn't `pub`, did you mean to add that?"

"Silly Christina, that's but an implementation detail. `Output` is the public interface."

You type up a simple example to demonstrate to her.
*/
use std::any::type_name;

pub const HELLO: &'static str = type_name::<Output<"Hello, world!", "world">>();

fn main() {
    dbg!(HELLO);
}
/*
"Now we simply compile, and observe the output."

You and Christina wait patiently for 38 seconds as rustc releases your code.

The terminal gladly tells you `HELLO` is `caw::Int<7>`.

"Well that's, impressive. Right, I was supposed to be interviewing you. We'll be in contact, for additional interviews. Have a good day!"

"Have a good day, Christina." You probably won't be seeing her again, you think as you glance towards the door.
*/
