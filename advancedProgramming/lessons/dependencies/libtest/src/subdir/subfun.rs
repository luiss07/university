
// neither these are directly accessible with a  `mod` declaration in `mod.rs`
// but this one is accessible with a `pub mod` declaration in `mod.rs`
pub fn pub_subfun() -> String {
    priv_subfun();  // only callable in this file
    return String::from("Subfun called");
}

fn priv_subfun() -> String {
    return String::from("Should not see this");
}

// technically a private enum, local to this file only
enum PrivSubEnum {
    TS1,
    TS2
}
// cannot be exported
// pub use PrivSubEnum as PSE;
pub enum PubSubEnum{
    PS1,
    PS2
}
// re-export with shorter name
pub use PubSubEnum as PSE;