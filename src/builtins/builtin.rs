pub trait BuiltinCommand {
    fn execute(&self, args: &[String]);
}
