use async_trait::async_trait;

#[async_trait(?Send)]
pub trait View: std::fmt::Debug {
    async fn render(&self);
}
