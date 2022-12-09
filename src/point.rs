use std::slice::SliceIndex;
use std::ops::IndexMut;
use std::ops::Index;
use std::str::FromStr;
use std::fmt::Display;

#[derive(Eq,PartialEq,Hash,Copy,Clone,Ord,PartialOrd,Debug)]
pub struct Point<const N: usize>(pub [isize;N]);

#[derive(Eq,PartialEq,Hash,Copy,Clone,Ord,PartialOrd,Debug)]
pub struct Offset<const N: usize>(pub [isize;N]);

impl<const N: usize> Point<N> {
    pub fn new(coords: &[isize]) -> Self {
        if coords.len() != N {
            panic!("invalid number of coordinates ({} instead of {})", coords.len(), N);
        }

        let mut point = Point([0;N]);
        for i in 0..N {
            point.0[i] = coords[i]
        }
        
        point
    }
}

impl<const N: usize> Offset<N> {
    pub fn new(coords: &[isize]) -> Self {
        if coords.len() != N {
            panic!("invalid number of coordinates ({} instead of {})", coords.len(), N);
        }

        let mut offset = Offset([0;N]);
        for i in 0..N {
            offset.0[i] = coords[i]
        }
        
        offset
    }
}

impl<const N: usize> std::ops::Sub for Point<N> {
    type Output = Offset<N>;
    fn sub(mut self, other: Point<N>) -> Offset<N> {
        for i in 0..N {
            self.0[i] -= other.0[i];
        }
        Offset(self.0)
    }
}

impl<const N: usize> std::ops::SubAssign<Offset<N>> for Point<N> {
    fn sub_assign(&mut self, other: Offset<N>) {
        for i in 0..N {
            self.0[i] -= other.0[i];
        }
    }
}

impl<const N: usize> std::ops::AddAssign<Offset<N>> for Point<N> {
    fn add_assign(&mut self, other: Offset<N>) {
        for i in 0..N {
            self.0[i] += other.0[i];
        }
    }
}

impl<const N: usize> std::ops::Add<Offset<N>> for Point<N> {
    type Output = Point<N>;
    fn add(mut self, other: Offset<N>) -> Point<N> {
        for i in 0..N {
            self.0[i] += other.0[i];
        }
        Point(self.0)
    }
}

impl<const N: usize> std::ops::Add for Offset<N> {
    type Output = Offset<N>;
    fn add(mut self, other: Offset<N>) -> Offset<N> {
        for i in 0..N {
            self.0[i] += other.0[i];
        }
        Offset(self.0)
    }
}

impl<const N: usize> std::ops::Sub for Offset<N> {
    type Output = Offset<N>;
    fn sub(mut self, other: Offset<N>) -> Offset<N> {
        for i in 0..N {
            self.0[i] -= other.0[i];
        }
        Offset(self.0)
    }
}

fn display_array<T>(items: &[T], f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>
where T: Display
{
    write!(f, "(")?;
    for i in 0..items.len() {
        if i != 0 {
            write!(f, ",")?;
        }
        write!(f, "{}", items[i])?;
    }
    write!(f, ")")
}

impl<const N: usize> Display for Point<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        display_array(&self.0, f)
    }
}

impl<const N: usize> Display for Offset<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        display_array(&self.0, f)
    }
}

impl<const N: usize> FromStr for Point<N> {
    type Err = ();
    fn from_str(s: &str) -> Result<Point<N>,()> {
        let coords = s.split(',').map(|s| s.parse().unwrap()).collect::<Vec<_>>();
        Ok(Point::new(&coords))
    }
}

impl<Idx, const N: usize> Index<Idx> for Point<N>
where Idx: SliceIndex<[isize]>
{
    type Output = Idx::Output;
    
    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl<Idx, const N: usize> IndexMut<Idx> for Point<N>
where Idx: SliceIndex<[isize]>
{
    fn index_mut(&mut self, i: Idx) -> &mut Self::Output {
         &mut self.0[i]
    }
}

impl<Idx, const N: usize> Index<Idx> for Offset<N>
where Idx: SliceIndex<[isize]>
{
    type Output = Idx::Output;
    
    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl<Idx, const N: usize> IndexMut<Idx> for Offset<N>
where Idx: SliceIndex<[isize]>
{
    fn index_mut(&mut self, i: Idx) -> &mut Self::Output {
         &mut self.0[i]
    }
}

impl<const N: usize> Default for Point<N>
{
    fn default() -> Self {
        Self([0;N])
    }
}

impl<const N: usize> Default for Offset<N>
{
    fn default() -> Self {
        Self([0;N])
    }
}