
//! parsing for SAP transaction COHV
//! 
//! COOIS might also work since they are similar

use calamine::DataType;

use std::collections::HashMap;
use std::path::PathBuf;

use crate::api::{Order, OrderData};
use super::excel::{XlsxTableReader, HeaderColumn};


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum CohvHeader {
    Order,
    Matl,
    Qty,
    Wbs,
    Type,
    Plant,
}

impl HeaderColumn for CohvHeader {
    type Row = Order;

    fn column_name(&self) -> String {
        use CohvHeader::*;
    
        match self {
            Order => "Order",
            Matl => "Material",
            Qty => "Qty",
            Wbs => "WBS Element",
            Type => "Order Type",
            Plant => "Plant"
        }.into()
    }

    fn columns_to_match() -> Vec<Self> where Self: Sized {
        vec![
            CohvHeader::Order,
            CohvHeader::Matl,
            CohvHeader::Qty,
            CohvHeader::Wbs,
            CohvHeader::Type,
            CohvHeader::Plant,
        ]
    }

    fn match_header_column(column_text: &str) -> Option<Self>
        where Self: Sized
    {
        match column_text {
            "Order"                  => Some( Self::Order ),
            "Material Number"        => Some( Self::Matl  ),
            "Order quantity (GMEIN)" => Some( Self::Qty   ),
            "WBS Element"            => Some( Self::Wbs   ),
            "Order Type"             => Some( Self::Type  ),
            "Plant"                  => Some( Self::Plant ),
            _                        => None
        }
    }

    fn parse_row(header: &HashMap<Self, usize>, row: &[DataType]) -> anyhow::Result<Self::Row>
        where Self: Sized
    {
        // TODO: handle parsing errors (get_string/get_int)
        let order = row[*header.get(&Self::Order).unwrap()].get_string().ok_or( anyhow!("Failed to coerce order to String") )?.parse()?;

        let matl  = row[*header.get(&Self::Matl).unwrap() ].get_string().ok_or( anyhow!("Failed to read Material as String") )?.into();
        let qty   = row[*header.get(&Self::Qty).unwrap()  ].get_float() .ok_or( anyhow!("Failed to read qty as Float") )? as u32;
        let wbs   = row[*header.get(&Self::Wbs).unwrap()  ].get_string().ok_or( anyhow!("Failed to read Wbs Element") )?.try_into()?;
        let _type = row[*header.get(&Self::Type).unwrap() ].get_string().ok_or( anyhow!("Failed to read Order Type") )?;
        let plant = row[*header.get(&Self::Plant).unwrap()].get_string().ok_or( anyhow!("Failed to read Plant") )?.into();

        let data = OrderData { id: order, mark: matl, qty, wbs, plant };

        Ok( Order::new(_type, data) )
    }
}

/// parses a COHV excel file from a given export file path
pub fn parse_cohv_xl(cohv_file: PathBuf) -> anyhow::Result<Vec<Order>> {
    
    let mut reader = XlsxTableReader::<CohvHeader>::new();
    let vals = reader.read_file(cohv_file)?
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();

    Ok(vals)
}
