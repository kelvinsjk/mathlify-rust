pub mod brackets;
pub use brackets::Brackets;

#[derive(Debug, Clone)]
pub enum Fn {
	Brackets(Brackets),
}
