# Writing specs in Rust

## Definitions

### Specification

A specification is "smaller" version of the system.

* Options
  * [Inductive specification](#inductive-specification)
  * [Specification-by-example](#specification-by-example)

#### Inductive specification

```
Sorted a r xs : Type
           xs : List a
        r x y : Type { x : a } { y : a } 
            a : Type
           
Sorted.Nil a r : Sorted a r (Nil a)

Sorted.Head a r head : Sorted a r (Cons a head (Nil a))
                head : a
                   
Sorted.Cons a r x y r_x_y tail y_tail_is_sorted : Sorted r a (Cons a x (Cons a y (tail)))
                                          r_x_y : r x y
                                           tail : List a
                               y_tail_is_sorted : Sorted (Cons a y (tail))                                  
```

#### Specification by example

```rust
use rand::Rng;

fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    
    let pivot = partition(arr);
    let (left, right) = arr.split_at_mut(pivot);
    
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot = arr.len() - 1;
    let mut i = 0;
    
    for j in 0..pivot {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, pivot);
    i
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_sorting_equivalence(vec: Vec<i32>) {
            let mut bubble_vec = vec.clone();
            let mut quick_vec = vec.clone();

            bubble_sort(&mut bubble_vec);
            quick_sort(&mut quick_vec);

            assert_eq!(bubble_vec, quick_vec);
        }
    }
}
```

## Notes

* The Rust developers won't understand the inductive specification
