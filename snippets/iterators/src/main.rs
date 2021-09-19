fn main() {
    let a1 = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    let a2 = [1, 3, 5, 7, 9, 11, 13];

    // Simplest use of iterator...
    println!("--------------------------------------------------");
    for e in a1.iter() {
        println!("item: {}", e);
    }

    // Chaining
    println!("--------------------------------------------------");
    // pub struct Chain<A, B> { /* fields omitted */ }
    // Chain: an iterator that links two iterators together, in a chain
    //
    // That struct is created by Iterator::chain
    // fn chain<U>(self, other: U) -> Chain<Self, <U as IntoIterator>::IntoIter>
    // where U: IntoIterator<Item = Self::Item>,
    // Takes two iterators and creates a new iterator over both in sequence.
    // chain() will return a new iterator which will first iterate over values
    // from the first iterator and then over values from the second iterator.
    // In other words, it links two iterators together, in a chain.
    // You can write fully in this way:
    // let iter: std::iter::Chain<std::iter::StepBy<_>, std::slice::Iter<_>> =
    //     a1.iter().step_by(2).chain(a2.iter());
    //
    // A cleaner way (move the use statements at the top of file):
    // use std::iter::Chain;
    // use std::iter::StepBy;
    // use std::slice::Iter;
    // let iter: Chain<StepBy<_>, Iter<_>> = a1.iter().step_by(2).chain(a2.iter());

    // the concise (preferred) way
    let iter = a1.iter().step_by(2).chain(a2.iter());

    for e in iter {
        println!("item: {}", e);
    }

    // Zipping
    println!("-----------------------------------x---------------");
    let iter = a2.iter().zip(a1.iter().step_by(2));
    for e in iter {
        println!("item: {:?}", e);
    }
}
