use crate::{
    datatype::{NFTError, NFTMetadata, RecognitionNFT, MAX_TITLE_LEN, MAX_DATE_LEN, MAX_TASK_LEN},
    interfaces::MetadataOperations,
    RecognitionSystemContract,
};
use soroban_sdk::{Address, Env, String};

#[allow(dead_code)]
impl MetadataOperations for RecognitionSystemContract {
    fn create_nft_metadata(
        organization: Address,
        title: String,
        date: String,
        task: String
    ) -> Result<NFTMetadata, NFTError> {
        // Validate organization address
        let org_str = organization.to_string();
        if org_str.len() == 0 {
            return Err(NFTError::InvalidAddress);
        }
        
        // Validate input lengths
        if title.len() as u32 > MAX_TITLE_LEN {
            return Err(NFTError::TitleTooLong);
        }
        if date.len() as u32 > MAX_DATE_LEN {
            return Err(NFTError::DateTooLong);
        }
        if task.len() as u32 > MAX_TASK_LEN {
            return Err(NFTError::TaskTooLong);
        }
        
        let metadata = NFTMetadata {
            ev_org: organization,
            ev_title: title,
            ev_date: date,
            task,
        };
        Ok(metadata)
    }

    fn update_metadata(
        env: &Env,
        admin: Address,
        token_id: u128,
        organization: Address,
        title: String,
        date: String,
        task: String,
    ) -> Result<(), NFTError> {
        // Check that admin is authorized
        admin.require_auth();
        
        // Verify admin is the actual contract admin
        let contract_admin = env.storage().instance().get(&crate::datatype::DataKeys::Admin)
            .ok_or(NFTError::UnauthorizedOwner)?;
        if admin != contract_admin {
            return Err(NFTError::UnauthorizedOwner);
        }

        // Validate organization address
        let org_str = organization.to_string();
        if org_str.len() == 0 {
            return Err(NFTError::InvalidAddress);
        }
        
        // Validate input lengths
        if title.len() as u32 > MAX_TITLE_LEN {
            return Err(NFTError::TitleTooLong);
        }
        if date.len() as u32 > MAX_DATE_LEN {
            return Err(NFTError::DateTooLong);
        }
        if task.len() as u32 > MAX_TASK_LEN {
            return Err(NFTError::TaskTooLong);
        }

        // Get the existing NFT
        let mut nft: RecognitionNFT = env
            .storage()
            .persistent()
            .get(&token_id)
            .ok_or(NFTError::BadgeNotFound)?;

        // Assign updated event fields
        nft.metadata.ev_title = title;
        nft.metadata.ev_date = date;
        nft.metadata.ev_org = organization;
        nft.metadata.task = task;

        env
            .storage()
            .persistent()
            .set(&token_id, &nft);
        Ok(())
    }
}
