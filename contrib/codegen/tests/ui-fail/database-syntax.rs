#[macro_use] extern crate rocket_contrib;

#[allow(unused_imports)]
use rocket_contrib::databases::diesel;

#[database]
struct A(diesel::SqliteConnection);

#[database(1)]
struct B(diesel::SqliteConnection);

#[database(123)]
struct C(diesel::SqliteConnection);

#[database("hello" "hi")]
struct D(diesel::SqliteConnection);

#[database("test")]
enum Foo {  }

#[database("test")]
struct Bar(diesel::SqliteConnection, diesel::SqliteConnection);

#[database("test")]
union Baz {  }

#[database("test")]
struct E<'r>(&'r str);

#[database("test")]
struct F<T>(T);

fn main() {  }
