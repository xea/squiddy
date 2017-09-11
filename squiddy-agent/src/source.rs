pub trait Source {
    type Item;

    fn next(&mut self) -> Self::Item;

}

/// `StdinSource` is a `Source` implementation that gets its items from the standard input.
pub struct StdinSource;

impl Source for StdinSource {
    type Item = String;

    fn next(&mut self) -> String {
        String::from("asf")
    }
}
