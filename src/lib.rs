use arbitrary::Arbitrary;
use lens_rs::*;

#[derive(Lens, Debug, Arbitrary)]
pub struct S {
    #[optic]
    e: E1,
    #[optic]
    x: String,
}

#[derive(Prism, Debug, Arbitrary)]
pub enum E1 {
    #[optic]
    A(SA),
    #[optic]
    B(SB),
}

#[derive(Lens, Debug, Arbitrary)]
pub struct SA {
    #[optic]
    x: Vec<SAX>,
    // #[optic]
    // y: HashMap<SAYK, SAYV>,
}

#[derive(Lens, Debug, Arbitrary)]
pub struct SB(u8);

#[derive(Lens, Debug, Arbitrary)]
pub struct SAX(#[optic] u8);

#[derive(Lens, std::hash::Hash, PartialEq, Eq, Debug, Arbitrary)]
pub struct SAYK(#[optic] u8);

#[derive(Lens, Debug, Arbitrary)]
pub struct SAYV(u8);

#[test]
fn example_lens_rs() {
    use lens_rs::*;

    let mut u = arbitrary::Unstructured::new(&[0]);

    let mut s: S = S::arbitrary(&mut u).unwrap();
    s.e = E1::A(SA {
        x: vec![SAX(1), SAX(2)],
    });

    assert_eq!(optics!(e.A.x._mapped._0).traverse(s), vec![1, 2]);
}
