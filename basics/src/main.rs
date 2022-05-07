// // use std::ops::Deref;

// // #[derive(Debug)]
// // struct MyBox<T>(T);

// // impl<T> Drop for MyBox<T> {
// //     fn drop(&mut self) {
// //         // CustomSmartPointerをデータ`{}`とともにドロップするよ
// //         println!("Dropping CustomSmartPointer with data!");
// //     }
// // }

// // impl<T> MyBox<T> {
// //     fn new(x: T) -> MyBox<T> {
// //         MyBox(x)
// //     }
// // }

// // impl<T> Deref for MyBox<T> {
// //     type Target = T;
// //     fn deref(&self) -> &T {
// //         &self.0
// //     }
// // }


// // fn hello(name: &str) {
// //     println!("Hello, {}!", name);
// // }

// // #![feature(conservative_impl_trait, universal_impl_trait)]

// pub fn twice<T>(f: impl Fn(T) -> T) -> impl Fn(T) -> T {
//     move |x| f(f(x))
// }

// // pub fn twice_dyn<T>(f: dyn Fn(T) -> T) -> impl Fn(T) -> T {
// //     move |x| f(f(x))
// // }


// // nの倍数を列挙
// fn multiples_of(n: i32) -> Box<Iterator<Item=i32>> {
//     if n == 0 {
//         Box::new(std::iter::once(0))
//     } else {
//         Box::new((0..).map(move |m| n * m))
//     }
// }

// fn main() {
//     // let _ff = twice(|x: i32| -> i32 { x + 1 });
//     // println!("{:?}", _ff(1));

//     // let f = twice(|x| x + 1)(1);
//     // println!("{}", f);

//     let x: Box<dyn Iterator<Item = i32>> = multiples_of(2);
//     for i in x.take(5) {
//         println!("{}", i);
//     }
//     // let y = multiples_of(5);
//     // println!("{:?}", *x(|i| i + 2));
// }

// use std::mem;
// use std::ptr;

// fn swap<T>(x: &mut T, y: &mut T) {
//     unsafe {
//         let mut temp: T = mem::MaybeUninit::uninit().assume_init();
//         //tempに一端値を退避して交換
//         ptr::copy_nonoverlapping(x, &mut temp, 1);
//         ptr::copy_nonoverlapping(y, x, 1);
//         ptr::copy_nonoverlapping(&temp, y, 1);
//     }
// }

// fn main() {
//     let mut x = Box::new(1); 
//     let mut y = Box::new(2);
//     swap(&mut x, &mut y);
//     println!("x = {}, y = {}", x, y)// x = 2, y = -46065496
// }

// fn main() -> anyhow::Result<()> {
//     Ok(())
// }

use anyhow;
use anyhow::Context;

fn foo() -> anyhow::Result<()> {
    bar().context("foo error")?;
    Ok(())
}

fn bar() -> anyhow::Result<()> {
    baz().context("bar error")?;
    Ok(())
}

fn baz() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("baz error"))
}

fn main() -> anyhow::Result<()> {
    if let Err(e) = foo() {
        println!("Err {}", e);
    }
    Ok(())
}