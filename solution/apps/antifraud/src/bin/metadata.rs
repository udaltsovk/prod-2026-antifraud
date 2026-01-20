use antifraud::AppConfig;
use lib::bootstrap::metadata::{
    DotenvExample, MetadataSaver as _, MetadataSaverResult,
};

fn main() -> MetadataSaverResult {
    let app_name = "antifraud";

    DotenvExample::<AppConfig>::default().save_as(app_name)?;

    Ok(())
}
