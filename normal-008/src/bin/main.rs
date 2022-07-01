use proconio::{input, fastout, marker::*};
use std::time::Instant;

make_struct! {
    #[derive(
        Debug, 
        Clone
    )]
    pub struct User {
        pub name: String,
        pub email: String,
        pub age: i32,
    }
}

fn main() {
    let start = Instant::now();

    let user_fields = User::fields();
    let user_field_types = User::types();

    let user = User {
        name: "mario".to_owned(),
        email: "test@email.com".to_owned(),
        age: 42,
    };

    println!("{:?}", user_fields);
    println!("{:?}", user_field_types);
    println!("{:?}", user.name);
    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

#[macro_export]
macro_rules! make_struct {
  (
    $(#[$attr:meta])*
    $pub:vis struct $name:ident { 
    $($fpub:vis $field:ident: $type:ty,)*  
  }) => {
    $(#[$attr])*
    $pub struct $name {
      $($fpub $field: $type,)*
    }
    impl $name {
      fn fields() -> Vec<&'static str> {
        vec![$(stringify!($field)),*]
      }

      fn types() -> Vec<&'static str> {
        vec![$(stringify!($type)),*]
      }
    }
  };
}
