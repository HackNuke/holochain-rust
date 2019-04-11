use crate::{context::Context, network::actions::get_links::get_links, nucleus,
workflows::get_entry_result::get_entry_with_meta_workflow};
use holochain_core_types::{chain_header::ChainHeader, time::Timeout};

use holochain_core_types::{
    cas::content::Address, crud_status::CrudStatus, entry::EntryWithMetaAndHeader,
    error::HolochainError,
};
use holochain_wasm_utils::api_serialization::get_links::{GetLinksResult,GetLinksArgs,LinksStatusRequestKind};
use std::sync::Arc;




pub async fn get_link_result_workflow<'a>(
    context: &'a Arc<Context>,
    link_args: &'a GetLinksArgs,
) -> Result<GetLinksResult, HolochainError> 
{
    let default_timeout = Timeout::default();
    let entry_with_workflow = await!(get_entry_with_meta_workflow(&context,&link_args.entry_address,&default_timeout))?;
    let headers = if link_args.options.sources
    {
        entry_with_workflow
        .ok_or(HolochainError::ErrorGeneric("Could not get entry".to_string()))?
        .headers
    }
    else
    {
        Vec::new()
    };

    if link_args.options.status_request != LinksStatusRequestKind::Live
    {
        Err(HolochainError::ErrorGeneric("Status rather than live not implemented".to_string()))
    }
    else
    {
        Ok(())
    }?;
 
    let links = await!(get_links(context.clone(),link_args.entry_address.clone(),link_args.tag.clone(),default_timeout))?;
    Ok(GetLinksResult::new(links,headers))
}