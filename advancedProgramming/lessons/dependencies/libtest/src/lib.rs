// tell the lib project that you're using a submodule
mod subdir;

pub fn gimme0()-> i32 {
    0
}
fn gimmeplusone(n:i32) -> i32{
    n+1
}

// created by intellij, discuss in the TESTING part
#[cfg(test)]
mod tests {
    mod testtest{
        pub fn asd(){

        }
        // write test for asd
    }
    use crate::{privmod, pubmod};
    use crate::*;

    #[test]
    fn test_gimme0(){
        let z = gimme0();
        assert_eq!(z,0);
    }
    #[test]
    fn test_gimmeplusone(){
        let o = crate::gimmeplusone(0);
        assert_eq!(o,1);
        let o = crate::gimmeplusone(10);
        assert_eq!(o,11);
    }

    // the test attribute
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        // tests often use the asser_eq! macro
        // and they have file-based access, so they'll have access to private things,
        //      like privmod
        // which are unaccessible from outside
        let _s = privmod::privmodfun();
        pubmod::pubmodfun();
        // QUIZ: will this work?
        // pubmod::pubmodprivfun();
        // Y/N

        // this is still unaccessible, it's behind another module, but if we place a test there, we can test it.
    }
}

/// a toplevel function that should be callable
pub fn toplevel_fun()-> String{
    let s = leaflib::leaflib();
    // use crate:: for absolute paths and for IDE suggestions
    crate::pubmod::pubmodfun();             // works
    //crate::pubmod::pubmodprivfun();       // does not work
    crate::privmod::privmodfun();           // works: the module is private to code outside this file
    crate::subdir::subfun::pub_subfun();
    // works so long as 'mod.rs' in 'subdir' says `pub mod` and not just `mod`
    // crate::subdir::subfun::priv_subfun(); //does not work
    return s;
}
mod integrationtests {
    #[test]
    fn test_integr(){
        let s = crate::toplevel_fun();
        assert_eq!(s,String::from("You just called leaflib"));
    }
}

// a private module that should not be visible
//  let's look at exported names too
mod privmod {
    // local import of a name exported by subfun
    use crate::subdir::subfun::{PSE, PubSubEnum};

    // even if this function is public, it is behind a private module
    pub fn privmodfun() -> String {
        let _e = PSE::PS1;
        // or we can use the already-available public name
        let _e = PubSubEnum::PS1;
        return String::from("can't see this");
    }
}

// a public module, should be visible
pub mod pubmod{

    // public function inside a public module, should be visible
    pub fn pubmodfun() -> String {
        pubmodprivfun();        // only callable here really, not in this file, in this module
        return String::from("pub mod pub fun");
    }
    // this private function should be local to the module
    fn pubmodprivfun() -> String {
        return String::from("can't see this too");
    }

    #[test]
    fn it_works_pubmodprivfun() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        pubmodprivfun();
    }
}


enum PrivEnum {
    T1,
    T2
}

pub enum PubEnum{
    P1,
    P2
}