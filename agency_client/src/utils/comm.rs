use crate::{httpclient, agency_settings};
use crate::utils::error::AgencyClientResult;

pub fn post_to_agency(body_content: &Vec<u8>) -> AgencyClientResult<Vec<u8>> {
    let endpoint = format!("{}/agency/msg", agency_settings::get_config_value(agency_settings::CONFIG_AGENCY_ENDPOINT)?);
    httpclient::post_message(body_content, &endpoint)
}
