use crate::client::Response;
use crate::Currency;
use crate::PayoutId;
use crate::ReportingReportRunId;
use crate::ReportingReportTypeId;
use crate::{resources::ReportingReportRun, Client};
use serde::Serialize;

impl ReportingReportRun {
    /// Creates a new object and begin running the report. (Certain report types require a live-mode API key.)
    ///
    /// For more details see <https://docs.stripe.com/api/reporting/report_run/create>
    pub fn create(client: &Client, params: CreateReportRunParams) -> Response<ReportingReportRun> {
        client.post_form("reporting/report_runs", params)
    }

    pub fn retrieve(
        client: &Client,
        reporting_run_id: ReportingReportRunId,
    ) -> Response<ReportingReportRun> {
        client.get(&format!("reporting/report_runs/{}", reporting_run_id))
    }
}

#[derive(Serialize, PartialEq, Debug, Clone, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub struct CreateReportRunParams<'a> {
    /// The ID of the report type to run, such as "balance.summary.1".
    pub report_type: ReportingReportTypeId,

    /// Parameters specifying how the report should be run.
    /// Different Report Types have different required and optional
    /// parameters, listed in the API Access to Reports documentation.
    pub parameters: CreateReportRunParamsInner<'a>,
}

#[derive(Serialize, PartialEq, Debug, Clone, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub struct CreateReportRunParamsInner<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_account: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_end: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<PayoutId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_category: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<&'a str>,
}

impl<'a> CreateReportRunParams<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}
