use std::fmt::Display;

trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn my_filter<P>(self, predicate: P) -> MyFilter<Self, P>
    where
        P: Fn(&Self::Item) -> bool,
        Self: Sized,
    {
        MyFilter {
            iterator: self,
            predicate,
        }
    }

    fn my_map<M, R>(self, mapper: M) -> MyMap<Self, M>
    where
        M: Fn(Self::Item) -> R,
        Self: Sized,
    {
        MyMap {
            iterator: self,
            mapper,
        }
    }

    fn my_sum<T>(mut self) -> T
    where
        Self: MyIterator<Item = T>,
        T: std::ops::Add<Output = T> + Default,
        Self: Sized,
    {
        let mut total = T::default();
        while let Some(v) = self.next() {
            total = total + v;
        }

        total
    }
}

impl<T> MyIterator for Vec<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(0))
        }
    }
}

struct MyFilter<I, P> {
    iterator: I,
    predicate: P,
}

impl<I, P> MyIterator for MyFilter<I, P>
where
    I: MyIterator,
    P: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.iterator.next() {
            if (self.predicate)(&v) {
                return Some(v);
            }
        }

        None
    }
}

struct MyMap<I, M> {
    iterator: I,
    mapper: M,
}

impl<I, M, R> MyIterator for MyMap<I, M>
where
    I: MyIterator,
    M: Fn(I::Item) -> R,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.iterator.next() {
            Some((self.mapper)(v))
        } else {
            None
        }
    }
}

fn print_iterator<T: Display>(mut iterator: impl MyIterator<Item = T>) {
    // Remember that MyIterator is not integrated to Rust
    // you will not be able to use `for elt in iterator {`
    let mut res = String::new();
    while let Some(el) = iterator.next() {
        res.push_str(&format!("{el},"));
    }

    println!("{res}");
}

fn main() {
    let enumeration = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    print_iterator(enumeration.clone());

    let filtered = enumeration.clone().my_filter(|&item| item % 2 == 0);
    print_iterator(filtered);

    let mapped = enumeration.clone().my_map(|item| format!("Value: {item}"));
    print_iterator(mapped);

    let total = enumeration.clone().my_sum();
    println!("Total: {total}");

    let filtered_mapped_total = enumeration
        .my_filter(|&item| item % 2 == 0)
        .my_map(|item| item * 2)
        .my_sum();
    println!("Filtered Mapped total is: {filtered_mapped_total}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect<T, I>(mut iterator: I) -> Vec<T>
    where
        I: MyIterator<Item = T>,
    {
        let mut res = Vec::new();
        while let Some(el) = iterator.next() {
            res.push(el);
        }

        res
    }

    #[test]
    fn test_filter() {
        let enumeration = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let filtered: Vec<i32> = collect(enumeration.my_filter(|&item| item % 2 == 0));
        assert_eq!(filtered, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_map() {
        let enumeration = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mapped: Vec<String> = collect(enumeration.my_map(|item| format!("Value: {item}")));
        assert_eq!(
            mapped,
            vec![
                "Value: 1",
                "Value: 2",
                "Value: 3",
                "Value: 4",
                "Value: 5",
                "Value: 6",
                "Value: 7",
                "Value: 8",
                "Value: 9",
                "Value: 10"
            ]
        );
    }

    #[test]
    fn test_total() {
        let enumeration = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let total = enumeration.my_sum();
        assert_eq!(total, 55);
    }

    #[test]
    fn test_filtered_mapped_total() {
        let enumeration = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let filtered_mapped_total = enumeration
            .my_filter(|&item| item % 2 == 0)
            .my_map(|item| item * 2)
            .my_sum();
        assert_eq!(filtered_mapped_total, 60);
    }
}
