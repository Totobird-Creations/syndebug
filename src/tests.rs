use crate::*;


#[derive(SynDebug)]
pub struct A {
    b  : u8,
    c  : u16,
    d  : D,
    e0 : E,
    e1 : E,
    e2 : E
}

#[derive(SynDebug)]
pub struct D {
    f : (),
    g : (Option<u32>, Option<bool>)
}

#[derive(SynDebug)]
pub enum E {
    H,
    I(&'static str),
    J {
        k : &'static [u8],
        l : [f32; 3]
    }
}

#[test]
fn simple_test() {
    let a = A {
        b : 123,
        c : 934,
        d : D {
            f : (),
            g : (None, Some(true),)
        },
        e0 : E::H,
        e1 : E::I("Hello!"),
        e2 : E::J {
            k : &[1, 2, 3,],
            l : [1.3, 943.456789, 4103.0,]
        }
    };
    assert_eq!(&to_string(&a), "");
}
