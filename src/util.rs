use agb::fixnum::FixedNum;

pub fn lerp<const N: usize>(a: FixedNum<N>, b: FixedNum<N>, t: FixedNum<N>) -> FixedNum<N> {
    a * (FixedNum::from(1) - t) + b * t
}
