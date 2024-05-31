use crate::client::Response;
use crate::List;
use crate::ReportingReportTypeId;
use crate::{resources::ReportingReportType, Client};

impl ReportingReportType {
    pub fn retrieve(
        client: &Client,
        reporting_type_id: ReportingReportTypeId,
    ) -> Response<ReportingReportType> {
        client.get(&format!("reporting/report_types/{}", reporting_type_id))
    }

    pub fn list(
        client: &Client,
    ) -> Response<List<ReportingReportType>> {
        client.get("reporting/report_types")
    }
}
