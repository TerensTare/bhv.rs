/// An enum type representing the outcome of calling [`Bhv::update`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    /// The behavior is still running.
    Running,
    /// The behavior was completed successfully.
    Success,
    /// The behavior failed to complete.
    Failure,
}

/// A trait used to denote that the implementing type can be used as
/// a behavior tree node.
pub trait Bhv {
    /// The context passed to the node every frame.
    type Context;

    /// Update the state of the behavior tree based on some global context.
    fn update(&mut self, ctx: &mut Self::Context) -> Status;

    /// Reset the node to initial status after completion, if needed.
    /// Defaults to nothing.
    fn reset(&mut self, _status: Status) {}

    /// Update the node until it returns a value different from [`Status::Running`].
    ///
    /// Useful for running a whole tree once built.
    ///
    /// If the node was executed successfully, returns `true`, `false` otherwise.
    fn execute(mut self, mut ctx: Self::Context) -> bool
    where
        Self: Sized,
    {
        loop {
            match self.update(&mut ctx) {
                Status::Running => continue,
                Status::Success => return true,
                Status::Failure => return false,
            }
        }
    }
}
