///Automatically generates all neccesary code to use a custom infix operator
///
///This is possible using a technique abusing operator overloading as first demonstrated
///by wishawa on [RFC #1579](https://github.com/rust-lang/rfcs/issues/1579#issuecomment-1398724803)
///
///# Examples
///
///Basic usage:
///
///```
///use infix_macro::infix;
///
///infix!(add, AddPartial, i32, i32, |a, b| a + b);
///
///fn main() {
///    let infix_adder = 5 *add* 5;
///    assert_eq!(10, infix_adder);
///}
///```
#[macro_export]
macro_rules! infix {
    ($name:ident, $partialname: ident, $input:ty, $output:ty, $function:expr) => {
        #[allow(non_camel_case_types)]
        pub struct $name;

        pub struct $partialname($input);

        impl std::ops::Mul<$name> for $input {
            type Output = $partialname;
            fn mul(self, _rhs: $name) -> Self::Output {
                $partialname(self)
            }
        }

        impl std::ops::Mul<$input> for $partialname {
            type Output = $output;
            fn mul(self, rhs: $input) -> Self::Output {
                $function(self.0, rhs)
            }
        }
    };
}
