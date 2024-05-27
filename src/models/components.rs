use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct Component {
    pub(crate) temperature: f32,
    pub(crate) max: f32,
    pub(crate) critical: Option<f32>,
    pub(crate) label: String,
}