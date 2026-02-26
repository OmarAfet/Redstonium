pub mod create;
pub mod instances;
pub mod settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePage {
    Instances,
    Create,
    Settings,
}
