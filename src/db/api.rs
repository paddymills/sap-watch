
/// represents the sql data for a part that was burned (PartArchive table)
#[derive(Debug)]
pub struct BurnedPart {
    /// The name of the part
    pub part: String,
    /// Quantity burned
    pub qty: i32,
    /// the material that the part(s) was burned from
    pub matl: MaterialData,
    /// the name of the program burned
    pub program: String
}

impl TryFrom<&tiberius::Row> for BurnedPart {
    type Error = tiberius::error::Error;

    fn try_from(row: &tiberius::Row) -> Result<Self, Self::Error> {
        let part = row.get::<&str, _>("Part").unwrap_or_default().into();
        let qty = row.get::<i32, _>("Qty").unwrap_or_default();
        let matl = MaterialData::try_from(row)?;
        let program = row.get::<&str, _>("Program").unwrap_or_default().into();
        
        Ok(Self { part, qty, matl, program })
    }
}

/// represents the material data (Stock/StockArchive table)
#[derive(Debug)]
pub struct MaterialData {
    /// the name of the material
    pub matl: String,
    /// the WBS element of the material (if non-stock)
    pub wbs: Option<String>,    // TODO: WBS struct
    /// the location the material is in
    pub loc: String,            // TODO: Location struct
    /// the plant the material is at
    pub plant: String,          // TODO: Plant struct
    /// the area of the material in question
    pub area: f64,
}

impl TryFrom<&tiberius::Row> for MaterialData {
    type Error = tiberius::error::Error;

    fn try_from(row: &tiberius::Row) -> Result<Self, Self::Error> {
        let matl = row.get::<&str, _>("MaterialMaster").unwrap_or_default().into();
        let wbs = row.get::<&str, _>("Wbs").map(Into::into);
        let loc = row.get::<&str, _>("Location").unwrap_or_default().into();
        let plant = row.get::<&str, _>("Plant").unwrap_or_default().into();
        let area = row.get::<f64, _>("Area").unwrap_or_default();

        Ok(Self { matl, wbs, loc, plant, area })
    }
}
