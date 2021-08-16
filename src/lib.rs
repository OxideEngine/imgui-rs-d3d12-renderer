#[derive(Debug, Clone)]
pub enum RendererError {}

pub struct Renderer;

impl Renderer {
    pub fn init(ctx: &mut imgui::Context) -> Result<Renderer, RendererError> {
        Ok(Renderer {})
    }
}
