#[cfg_attr(test, derive(Debug, PartialEq))]
enum Tree<T> {
  Leaf(T),
  Branch(Box<Tree<T>>, Box<Tree<T>>)
}

impl<T> Tree<T> {
  fn leaf(val: T) -> Tree<T> {
    Tree::Leaf(val)
  }

  fn branch(left: T, right: T) -> Tree<T> {
    Tree::Branch(
      Box::new(Tree::leaf(left)),
      Box::new(Tree::leaf(right))
    )
  }
}

type Result<T> = std::result::Result<T, ()>;

impl <T: PartialOrd + Default> Tree<T> {
  fn to_branch_with(&mut self, other: T) -> Result<()> {
    use Tree::Leaf;

    let this_val = {
      if let &mut Leaf(ref mut val) = self {
        std::mem::replace(val, T::default())
      }
      else {
        return Err(());
      }
    };

    if this_val <= other {
      *self = Tree::branch(this_val, other);
    }
    else {
      *self = Tree::branch(other, this_val);
    }

    Ok(())
  }
}


fn find_mut<'a, T, F>(start: &'a mut Tree<T>, mut f: F) -> Option<&'a mut Tree<T>>
  where F: FnMut(&T) -> bool,
        T: PartialOrd
{
  use Tree::{Leaf, Branch};

  fn find_impl<'a, T, F>(node: &'a mut Tree<T>, f: &mut F) -> Option<&'a mut Tree<T>>
    where F: FnMut(&T) -> bool,
          T: PartialOrd
  {
    match node {
      &mut Leaf(ref value) => { 
        if !f(value) { 
          return None;
        } 
      },
      &mut Branch(ref mut left, ref mut right) => {
        if let Some(n) = find_impl(left, f) {
          return Some(n);
        }

        return find_impl(right, f)
      }
    }

    Some(node)
  }

  find_impl(start, &mut f)
}

fn find<'a, T, F>(start: &'a Tree<T>, mut f: F) -> Option<&'a Tree<T>>
  where F: FnMut(&T) -> bool,
        T: PartialOrd
{
  use Tree::{Leaf, Branch};

  fn find_impl<'a, T, F>(node: &'a Tree<T>, f: &mut F) -> Option<&'a Tree<T>>
    where F: FnMut(&T) -> bool,
          T: PartialOrd
  {
    match node {
      &Leaf(ref value) => { 
        if f(value) { 
          return Some(node); 
        } 

        return None;
      },
      &Branch(ref left, ref right) => {
        if let Some(n) = find_impl(left, f) {
          return Some(n);
        }

        find_impl(right, f)
      }
    }
  }

  find_impl(start, &mut f)
}

fn descend<T, F>(start: &Tree<T>, mut f: F)
  where F: FnMut(&T)
{
  use Tree::{Leaf, Branch};

  fn descend_impl<T, F>(s: &Tree<T>, f: &mut F)
    where F: FnMut(&T)
  {
    match s {
      &Leaf(ref value) => f(value),
      &Branch(ref left, ref right) => {
        descend_impl(left, f);
        descend_impl(right, f);
      }
    }
  }

  descend_impl(start, &mut f);
}

#[cfg(test)]
mod tests {
  use super::{
    Tree,
    descend,
    find,
    find_mut
  };

  #[test]
  fn create_new_should_have_one_element() {
    let t = Tree::leaf(42);

    let mut n = 0;

    descend(&t, |_| n += 1);

    assert_eq!(n, 1);
  }

  #[test]
  fn test_find() {
    let t = Tree::leaf(42);

    let n = find(&t, |v| *v == 42);

    assert!(n.is_some());
    assert_eq!(Tree::Leaf(42), *n.unwrap());
  }

  #[test]
  fn test_find_mut() {
    let mut t = Tree::leaf(42);

    {
      let n = find_mut(&mut t, |v| *v > 15).unwrap();
      n.to_branch_with(15).unwrap();
    }

    {
      let n = find_mut(&mut t, |v| *v <= 30).unwrap();
      n.to_branch_with(30).unwrap();
    }

    let mut v = vec![];
    descend(&t, |value| v.push(*value));

    assert_eq!(v, vec![15, 30, 42]);
  }
}
