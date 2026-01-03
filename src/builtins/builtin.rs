pub trait BuiltinCommand {
    fn execute(&self, args: &[&str]);
}
