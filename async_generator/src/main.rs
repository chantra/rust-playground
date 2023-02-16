#![feature(generators)]
use anyhow::Result;
use async_stream::try_stream;
use futures::pin_mut;
use futures::stream::Stream;
use futures::StreamExt;
use futures::TryStreamExt;

struct F {}

impl F {
    fn four_to_seven<'a>(&'a self, s: &'a str) -> impl Stream<Item = Result<u32>> + '_ {
        try_stream! {
            println!("{}", s);
            for i in 4..7 {
                yield i;
            }
        }
    }

    fn zero_to_three<'a>(&'a self, s: &'a str) -> impl Stream<Item = Result<u32>> + '_ {
        try_stream! {
            println!("{}", s);
            for i in 0..3 {
                yield i;
            }
        }
    }

    fn zero_to_seven<'a>(&'a self) -> impl Stream<Item = Result<u32>> + '_ {
        try_stream! {
            let s1 = self.zero_to_three("zero");
            pin_mut!(s1); // needed for iteration

            let s2 = self.four_to_seven("four");
            pin_mut!(s2);

            let s= s1.chain(s2);
            pin_mut!(s);

            while let Some(x) = s.try_next().await? {
                yield x;
            }

        }
    }
}
#[tokio::main]
async fn main() {
    let f = F {};

    let s = f.zero_to_seven();
    pin_mut!(s);
    while let Some(value) = s.try_next().await.unwrap() {
        println!("got {}", value);
    }
}
