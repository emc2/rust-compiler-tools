use compiler_tools::nondistinct::Nondistinct;
use compiler_tools::position::BasicPosition;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
enum Exp<'a> {
    Num { val: usize, pos: Nondistinct<BasicPosition<'a>> },
    Plus { left: Box<Exp<'a>>, right: Box<Exp<'a>>,
           pos: Nondistinct<BasicPosition<'a>> }
}

#[test]
fn test_eq_same_pos() {
    let pos_1 = BasicPosition::Synthetic { desc: "a".to_string() };
    let a = Exp::Plus {
        left: Box::new(Exp::Num { pos: Nondistinct::from(pos_1.clone()),
                                  val: 2 }),
        right: Box::new(Exp::Num { pos: Nondistinct::from(pos_1.clone()),
                                   val: 1 }),
        pos: Nondistinct::from(pos_1.clone())
    };
    let b = Exp::Plus {
        left: Box::new(Exp::Num { pos: Nondistinct::from(pos_1.clone()),
                                  val: 2 }),
        right: Box::new(Exp::Num { pos: Nondistinct::from(pos_1.clone()),
                                   val: 1 }),
        pos: Nondistinct::from(pos_1.clone())
    };

    assert_eq!(a, b);
}

#[test]
fn test_eq_diff_pos() {
    let pos_1 = BasicPosition::Synthetic { desc: "a".to_string() };
    let pos_2 = BasicPosition::Synthetic { desc: "b".to_string() };
    let a = Exp::Plus {
        left: Box::new(Exp::Num { pos: Nondistinct::from(pos_1.clone()),
                                  val: 2 }),
        right: Box::new(Exp::Num { pos: Nondistinct::from(pos_1.clone()),
                                   val: 1 }),
        pos: Nondistinct::from(pos_1.clone())
    };
    let b = Exp::Plus {
        left: Box::new(Exp::Num { pos: Nondistinct::from(pos_2.clone()),
                                  val: 2 }),
        right: Box::new(Exp::Num { pos: Nondistinct::from(pos_2.clone()),
                                   val: 1 }),
        pos: Nondistinct::from(pos_2.clone())
    };

    assert_eq!(a, b);
}

#[test]
fn test_ne_same_pos() {
    let pos_1 = BasicPosition::Synthetic { desc: "a".to_string() };
    let a = Exp::Plus {
        left: Box::new(Exp::Num { pos: Nondistinct::from(pos_1.clone()),
                                  val: 2 }),
        right: Box::new(Exp::Num { pos: Nondistinct::from(pos_1.clone()),
                                   val: 1 }),
        pos: Nondistinct::from(pos_1.clone())
    };
    let b = Exp::Plus {
        left: Box::new(Exp::Num { pos: Nondistinct::from(pos_1.clone()),
                                  val: 2 }),
        right: Box::new(Exp::Num { pos: Nondistinct::from(pos_1.clone()),
                                   val: 3 }),
        pos: Nondistinct::from(pos_1.clone())
    };

    assert_ne!(a, b);
}

#[test]
fn test_ne_diff_pos() {
    let pos_1 = BasicPosition::Synthetic { desc: "a".to_string() };
    let pos_2 = BasicPosition::Synthetic { desc: "b".to_string() };
    let a = Exp::Plus {
        left: Box::new(Exp::Num { pos: Nondistinct::from(pos_1.clone()),
                                  val: 2 }),
        right: Box::new(Exp::Num { pos: Nondistinct::from(pos_1.clone()),
                                   val: 1 }),
        pos: Nondistinct::from(pos_1.clone())
    };
    let b = Exp::Plus {
        left: Box::new(Exp::Num { pos: Nondistinct::from(pos_2.clone()),
                                  val: 2 }),
        right: Box::new(Exp::Num { pos: Nondistinct::from(pos_2.clone()),
                                   val: 3 }),
        pos: Nondistinct::from(pos_2.clone())
    };

    assert_ne!(a, b);
}
