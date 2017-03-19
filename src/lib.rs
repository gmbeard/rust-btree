#[cfg_attr(test, derive(Debug, PartialEq))]
struct Tree<T> {
  value: T,
  left: Option<Box<Tree<T>>>,
  right: Option<Box<Tree<T>>>
}

impl<T> Tree<T> {
  fn leaf(val: T) -> Tree<T> {
    Tree { value: val, left: None, right: None }
  }

  fn branch(val: T, left: T, right: T) -> Tree<T> {
    Tree { 
      value: val,
      left: Some(Box::new(Tree::leaf(left))),
      right: Some(Box::new(Tree::leaf(right)))
    }
  }

  fn left(&self) -> Option<&Tree<T>> {
    match self.left.as_ref() {
      Some(n) => Some(&*n),
            _ => None
    }
  }

  fn right(&self) -> Option<&Tree<T>> {
    match self.right.as_ref() {
      Some(n) => Some(&*n),
            _ => None
    }
  }
}

impl<T: PartialOrd> Tree<T> {
  fn push(&mut self, value: T) {
    let node: &mut Option<Box<Tree<T>>>;

    if value <= self.value {
      node = &mut self.left;
    }
    else {
      node = &mut self.right;
    }

    if let Some(n) = node.as_mut() {
      n.push(value);
      return;
    }

    *node = Some(Box::new(Tree::leaf(value)));
  }
}

fn find_mut<'a, T, F>(start: &'a mut Tree<T>, mut f: F) -> Option<&'a mut Tree<T>>
  where F: FnMut(&T) -> bool,
        T: PartialOrd
{
  fn find_impl<'a, T, F>(node: &'a mut Tree<T>, f: &mut F) -> Option<&'a mut Tree<T>>
    where F: FnMut(&T) -> bool,
          T: PartialOrd
  {
    let found = if let Some(node) = node.left.as_mut() {
      find_impl(node, f).is_some()
    }
    else {
      false
    };

    if found {
      return find_impl(node.left.as_mut().unwrap(), f);
    }
    else {
      if f(&node.value) {
        return Some(node);
      }
    }

    if let Some(node) = node.right.as_mut() {
      return find_impl(node, f);
    }

    None
  }

  find_impl(start, &mut f)
}

fn find<'a, T, F>(start: &'a Tree<T>, mut f: F) -> Option<&'a Tree<T>>
  where F: FnMut(&T) -> bool,
        T: PartialOrd
{
  fn find_impl<'a, T, F>(node: &'a Tree<T>, f: &mut F) -> Option<&'a Tree<T>>
    where F: FnMut(&T) -> bool,
          T: PartialOrd
  {
    if let Some(node) = node.left.as_ref() {
      if let Some(n) = find_impl(node, f) {
        return Some(n);
      }
    }

    if f(&node.value) {
      return Some(node);
    }

    if let Some(node) = node.right.as_ref() {
      if let Some(n) = find_impl(node, f) {
        return Some(n);
      }
    }

    None
  }

  find_impl(start, &mut f)
}


fn descend<T, F>(start: &Tree<T>, mut f: F)
  where F: FnMut(&T)
{
  fn descend_impl<T, F>(s: &Tree<T>, f: &mut F)
    where F: FnMut(&T)
  {
    if s.left.is_some() {
      descend_impl(s.left.as_ref().unwrap(), f);
    }

    f(&s.value);

    if s.right.is_some() {
      descend_impl(s.right.as_ref().unwrap(), f);
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
    assert_eq!(42, n.unwrap().value);
  }

  #[test]
  fn test_push_results_in_correct_order() {
    let mut t = Tree::leaf(42);
    t.push(15);
    t.push(30);

    let mut v = vec![];
    descend(&t, |value| v.push(*value));

    assert_eq!(v, vec![15, 30, 42]);
  }
}
