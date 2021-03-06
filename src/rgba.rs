//! `rgba` contains various data structures for working in the RGBA color space.
//!

extern crate minivec;

use crate::Numeric;

/// `Image` represents any `RGBA` image. Internally, it stores each channel as an independent
/// allocation which enables such things as constant-time channel swapping along with making the
/// data cheaper to copy to a GPU which expects `CHW` ordering vs the packed format `HWC`.
///
#[derive(Default)]
pub struct Image<T>
where
  T: Numeric,
{
  pub(super) r: minivec::MiniVec<T>,
  pub(super) g: minivec::MiniVec<T>,
  pub(super) b: minivec::MiniVec<T>,
  pub(super) a: minivec::MiniVec<T>,
  pub(super) h: usize,
  pub(super) w: usize,
}

impl<T> Image<T>
where
  T: Numeric,
{
  /// `r` returns an immutable reference to the image's red channel as a `&[T]`.
  ///
  #[must_use]
  pub fn r(&self) -> &[T] {
    self.r.as_slice()
  }

  /// `g` returns an immutable reference to the image's green channel as a `&[T]`.
  ///
  #[must_use]
  pub fn g(&self) -> &[T] {
    self.g.as_slice()
  }

  /// `b` returns an immutable reference to the image's blue channel as a `&[T]`.
  ///
  #[must_use]
  pub fn b(&self) -> &[T] {
    self.b.as_slice()
  }

  /// `a` returns an immutable reference to the image's alpha channel as a `&[T]`.
  ///
  #[must_use]
  pub fn a(&self) -> &[T] {
    self.a.as_slice()
  }

  /// `width` returns the number of columns in the image.
  ///
  #[must_use]
  pub fn width(&self) -> usize {
    self.w
  }

  /// `height` returns the number of rows in the image.
  ///
  #[must_use]
  pub fn height(&self) -> usize {
    self.h
  }

  /// `rgba_iter` returns an iterator that traverses the planar image data in a row-major ordering, yielding each pixel
  /// as a `[T; 4]`.
  ///

  pub fn rgba_iter(&self) -> impl Iterator<Item = [T; 4]> + '_ {
    make_iter(&self.r, &self.g, &self.b, &self.a)
  }

  /// `rgb_iter` returns an iterator that traverses the planar image data in a row-major ordering, yielding each pixel
  /// as a `[T; 3]`.
  ///
  pub fn rgb_iter(&self) -> impl Iterator<Item = [T; 3]> + '_ {
    crate::rgb::make_iter(&self.r, &self.g, &self.b)
  }
}

/// `make_iter` returns an iterator that traverses the planar image data in a row-major ordering, yielding each pixel
/// as a `[T; 4]`.
///
pub fn make_iter<'a, T: Numeric>(
  r: &'a [T],
  g: &'a [T],
  b: &'a [T],
  a: &'a [T],
) -> impl Iterator<Item = [T; 4]> + 'a {
  r.iter()
    .copied()
    .zip(g.iter().copied())
    .zip(b.iter().copied())
    .zip(a.iter().copied())
    .map(|(((x, y), z), w)| [x, y, z, w])
}

/// `make_iter_mut` returns an iterator that traverses the planar image data in a row-major ordering, yielding each
/// pixel as a `[&mut T; 4]` so that the underlying pixel values can be manipulated.
///
pub fn make_iter_mut<'a, T: Numeric>(
  r: &'a mut [T],
  g: &'a mut [T],
  b: &'a mut [T],
  a: &'a mut [T],
) -> impl Iterator<Item = [&'a mut T; 4]> {
  r.iter_mut()
    .zip(g.iter_mut())
    .zip(b.iter_mut())
    .zip(a.iter_mut())
    .map(|(((x, y), z), w)| [x, y, z, w])
}
