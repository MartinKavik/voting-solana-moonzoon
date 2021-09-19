use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pub_key::Pubkey,
};
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Escrow {
    pub is_initialized: bool,
    pub initializer_pub_key: Pubkey,
    pub temp_token_account_pub_key: Pubkey,
    pub initializer_token_to_receive_account_pub_key: Pubkey,
    pub expected_amount: u64,
}

impl Sealed for Escrow {}

impl IsInitialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Escrow {
    const LEN: usize = 105;  // 1 (bool) + 3 * 32 (Pubkey) + 1 * 8 (u64)
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Escrow::LEN];
        let (
            is_initialized,
            initializer_pub_key,
            temp_token_account_pub_key,
            initializer_token_to_receive_account_pub_key,
            expected_amount,
        ) = array_refs![src, 1, 32, 32, 32, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(Escrow {
            is_initialized,
            initializer_pub_key: Pubkey::new_from_array(*initializer_pub_key),
            temp_token_account_pub_key: Pubkey::new_from_array(*temp_token_account_pub_key),
            initializer_token_to_receive_account_pub_key: Pubkey::new_from_array(*initializer_token_to_receive_account_pub_key),
            expected_amount: u64::from_le_bytes(*expected_amount),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Escrow::LEN];
        let (
            is_initialized_dst,
            initializer_pub_key_dst,
            temp_token_account_pub_key_dst,
            initializer_token_to_receive_account_pub_key_dst,
            expected_amount_dst,
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8];

        let Escrow {
            is_initialized,
            initializer_pub_key,
            temp_token_account_pub_key,
            initializer_token_to_receive_account_pub_key,
            expected_amount,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        initializer_pub_key_dst.copy_from_slice(initializer_pub_key.as_ref());
        temp_token_account_pub_key_dst.copy_from_slice(temp_token_account_pub_key.as_ref());
        initializer_token_to_receive_account_pub_key_dst.copy_from_slice(initializer_token_to_receive_account_pub_key.as_ref());
        *expected_amount_dst = expected_amount.to_le_bytes();
    }
}
