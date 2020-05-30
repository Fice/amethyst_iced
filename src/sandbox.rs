use crate::backend::IcedRenderer;
use iced_native::Hasher;

pub type Element<'a, 'r, Message> = iced_native::Element<'a, Message, IcedRenderer<'r>>;

/// The Sandbox is a basic UI wrapper.
///
/// This trait should be implemented by your UI State and defines
/// how the UI reacts to messages.
///
/// When receiving an UI message, the user can optionally send back a GameMessage
/// that their systems will listen to and react accordingly, allowing ECS interaction with the UI.
//
// Note: UIMessage & GameMessage have to be different types, otherwise the application will crash.
pub trait Sandbox: Send + Sync + 'static {
    type UIMessage: Send + Sync + 'static;
    type GameMessage: Send + Sync + 'static;

    fn update(&mut self, _message: &Self::UIMessage) -> Vec<Self::GameMessage> {
        vec![]
    }

    fn view(&mut self) -> Element<Self::UIMessage>;

    fn hash_layout(&mut self, state: &mut Hasher) {
        self.view().hash_layout(state)
    }
}

#[derive(Default)]
/// The SandboxContainer is the structure that will store the Sandbox in the
/// ECS environment
pub struct SandboxContainer<S: Sandbox>(S);

impl<S: Sandbox> SandboxContainer<S> {
    pub fn new(sandbox: S) -> Self {
        SandboxContainer(sandbox)
    }
}

impl<S: Sandbox> Sandbox for SandboxContainer<S> {
    type UIMessage = S::UIMessage;
    type GameMessage = S::GameMessage;

    fn update(&mut self, message: &S::UIMessage) -> Vec<S::GameMessage> {
        self.0.update(message)
    }

    fn view(&mut self) -> Element<S::UIMessage> {
        self.0.view()
    }
}
