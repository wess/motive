//
// macro.rs
// motive
// 
// Author: wess (me@wess.io)
// Created: 02/18/2022
// 
// Copywrite (c) 2022 Wess.io
//

#[macro_export]
macro_rules! am {
  ($var:expr) => (Arc::new(Mutex::new($var));)
}