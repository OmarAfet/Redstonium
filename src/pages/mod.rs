pub mod create;
pub mod instances;
pub mod settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePage {
    Instances,
    Create,
    Settings,
    // Temporary tabs for scroll testing
    TempTab1,
    TempTab2,
    TempTab3,
    TempTab4,
    TempTab5,
    TempTab6,
    TempTab7,
    TempTab8,
    TempTab9,
    TempTab10,
    TempTab11,
    TempTab12,
}
