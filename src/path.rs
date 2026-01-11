use crate::*;

pub trait PathExt {
  #[allow(private_interfaces)]
  const SEALED: Sealed;

  fn last_segment(&self) -> &PathSegment;
  fn last_segment_mut(&mut self) -> &mut PathSegment;
  fn leading_path(&self) -> Vec<&PathSegment>;
}

impl PathExt for Path {
  #[allow(private_interfaces)]
  const SEALED: Sealed = Sealed;

  fn leading_path(&self) -> Vec<&PathSegment> {
    let mut segments: Vec<&PathSegment> = Vec::new();

    let mut segments_iter = self.segments.iter().peekable();

    while let Some(segment) = segments_iter.next() {
      if segments_iter.peek().is_some() {
        segments.push(segment);
      }
    }

    segments
  }

  #[inline]
  fn last_segment(&self) -> &PathSegment {
    self.segments.last().unwrap()
  }

  #[inline]
  fn last_segment_mut(&mut self) -> &mut PathSegment {
    self.segments.last_mut().unwrap()
  }
}
