pub use sea_orm_migration::prelude::*;

mod m20260709_010051_create_valuation_and_bazin_inputs;
mod m20260709_152010_create_graham_inputs;
mod m20260709_153134_create_gordon_inputs;
mod m20260709_154525_create_dcf_inputs;
mod m20260709_155420_create_banks_inputs;
mod m20260709_160307_create_rnav_inputs;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260709_010051_create_valuation_and_bazin_inputs::Migration),
            Box::new(m20260709_152010_create_graham_inputs::Migration),
            Box::new(m20260709_153134_create_gordon_inputs::Migration),
            Box::new(m20260709_154525_create_dcf_inputs::Migration),
            Box::new(m20260709_155420_create_banks_inputs::Migration),
            Box::new(m20260709_160307_create_rnav_inputs::Migration),
        ]
    }
}
