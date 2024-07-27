use crate::Vector2D;

use agb::fixnum::FixedNum;

pub trait CloseToZero<T>
where T: Ord + PartialOrd
{
   fn close_to_zero(&self, precision: T) -> bool;
}

impl CloseToZero<FixedNum<8>> for FixedNum<8>
{
   fn close_to_zero(&self, precision: FixedNum<8>) -> bool {
      *self < precision && *self > -precision
   }
}

impl CloseToZero<FixedNum<8>> for Vector2D<FixedNum<8>>
{
   fn close_to_zero(&self, precision: FixedNum<8>) -> bool {
        self.x.close_to_zero(precision) && self.y.close_to_zero(precision)
   }
}

