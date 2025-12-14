use crate::*;

pub trait PathExt {
  fn last_segment(&self) -> &PathSegment;
  fn last_segment_mut(&mut self) -> &mut PathSegment;
  fn leading_path(&self) -> Vec<&PathSegment>;
}

impl PathExt for Path {
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

  fn last_segment(&self) -> &PathSegment {
    self.segments.last().unwrap()
  }

  fn last_segment_mut(&mut self) -> &mut PathSegment {
    self.segments.last_mut().unwrap()
  }
}
