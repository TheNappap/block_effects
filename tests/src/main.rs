
use block_effects::block;
use futures::executor;

#[test]
fn unsafe_() {
    block!{ 
        let val = unsafe {
            let ref_: *const i32 = &32; 
            *ref_
        };
    }
    assert_eq!(val,32);
}

#[test]
fn loop_() {
    block!{ 
        let did_loop = loop {
            break true;
        };
    }
    assert!(did_loop);
}

#[test]
fn unsafe_loop() {
    block!{ 
        let val = unsafe loop {
            let ref_: *const i32 = &32; 
            break *ref_;
        };
    }
    assert_eq!(val,32);
}

#[test]
fn async_() {
    block!{ 
        let f = async {
            1+2
        };
    }
    assert_eq!(executor::block_on(f),3);
}

#[test]
fn async_unsafe_loop() {
    block!{ 
        let f = async unsafe loop {
            let ref_: *const i32 = &32; 
            break *ref_;
        };
    }
    assert_eq!(executor::block_on(f),32);
}

#[test]
fn if_() {
    block!{ 
        let mut did_if = false;
        if true || false {
            did_if = true
        };
    }
    assert!(did_if);
}

#[test]
fn if_let() {
    block! {
        let mut did_if = false;
        let opt = Some(3);
        if let Some(i) = opt {
            assert_eq!(i,3);
            did_if = true;
        }
    }
    assert!(did_if);
}

#[test]
fn if_else() {
    block!{ 
        let did_if = if true && false {
            assert!(false);
            true
        } else {
            assert!(true);
            true
        };
    }
    assert!(did_if);
}

#[test]
fn else_if() {
    block!{ 
        let did_if = if true && false {
            assert!(false);
            true
        } else if true  {
            assert!(true);
            true
        } else { 
            assert!(false);
            true 
        };
    }
    assert!(did_if);
}

#[test]
fn while_() {
    block!{ 
        let mut did_while = false;
        while true || false {
            did_while = true;
            break;
        }
    }
    assert!(did_while);
}

#[test]
fn while_let() {
    block!{ 
        let mut did_while = false;
        let opt = Some(3);
        while let Some(i) = opt {
            assert_eq!(i,3);
            did_while = true;
            break;
        }
    }
    assert!(did_while);
}

#[test]
fn for_() {
    block!{ 
        let mut did_for = false;
        for i in 0..3 {
            assert!(i < 3);
            did_for = true;
        }
    }
    assert!(did_for);
}

#[test]
fn match_() {
    block!{ 
        let did_match = match 1 {
            0 => false,
            _ => true
        };
    }
    assert!(did_match);
}

#[test]
fn for_while_if() {
    block!{ 
        let mut did_block = false;
        let do_while = true;
        let do_if = true;
        for i in 0..3 while do_while if do_if {
            assert!(i < 3);
            did_block = true;
            break;
        }
    }
    assert!(did_block);
}

#[test]
fn for_for() {
    block!{ 
        let mut did_for = false;
        for i in 0..3 for j in 0..3 {
            assert!(i < 3);
            assert!(j < 3);
            did_for = true;
        }
    }
    assert!(did_for);
}

#[test]
fn if_let_if_let() {
    block!{  
        let mut did_if = false;
        let opt = Some(3);
        if let Some(i) = opt if let Some(j) = opt {
            assert_eq!(i,3);
            assert_eq!(j,3);
            did_if = true;
        }
    }
    assert!(did_if);
}

#[test]
fn if_let_while_let() {
    block!{ 
        let mut did_block = false;
        let opt = Some(3);
        if let Some(i) = opt while let Some(j) = opt {
            assert_eq!(i,3);
            assert_eq!(j,3);
            did_block = true;
            break;
        }
    }
    assert!(did_block);
}

#[test]
fn if_else_match() {
    block!{ 
        let did_match = if true || false match 0 {
            0 => true,
            _ => false
        } else {
            assert!(false);
            false
        };
    }
    assert!(did_match);
}

#[test]
fn multiple_blocks() {
    block!{ 
        let mut did_for = false;
        let mut did_while = false;
        let mut did_if = false;
        for i in 0..3 for j in 0..3 {
            assert!(i < 3);
            assert!(j < 3);
            did_for = true;
        }
        while true || false {
            did_while = true;
            break;
        }
        if true match 0 {
            0 => did_if = true,
            _ => ()
        } else {
            assert!(false);
        }
    }
    assert!(did_for);
    assert!(did_while);
    assert!(did_if);
}


#[test]
fn multiple_blocks2() {
    block!{ 
        let mut did_for = false;
        let mut did_if = false;
        let mut did_while = false;
        for i in 0..3 match i { 
            2 => { 
                assert!(i < 3); 
                did_for = true;
            },
            _ => assert!(i < 3) 
        }
        if true unsafe { 
            let ref_: *const i32 = &32; 
            did_if = *ref_ == 32
        }
        while let Some(x) = Some(32) match x { 
            32 => { 
                did_while = true; 
                break; 
            },
            _ => assert!(false)  
        }
    }
    assert!(did_for);
    assert!(did_while);
    assert!(did_if);
}