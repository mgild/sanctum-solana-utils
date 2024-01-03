use std::{error::Error, fmt::Display, hash::Hash, ops::Deref};

use solana_program::pubkey::Pubkey;
use solana_readonly_account::{
    ReadonlyAccountData, ReadonlyAccountIsExecutable, ReadonlyAccountLamports,
    ReadonlyAccountOwner, ReadonlyAccountRentEpoch,
};

pub const SMALL_ACCOUNT_DATA_MAX_LEN: u8 = 15;

pub const SMALL_ACCOUNT_DATA_MAX_LEN_USIZE: usize = SMALL_ACCOUNT_DATA_MAX_LEN as usize;

/// An account with data len < SMALL_ACCOUNT_DATA_MAX_LEN
/// that stores its data inline
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SmallAccount {
    data: [u8; SMALL_ACCOUNT_DATA_MAX_LEN_USIZE], // data first so that it's always 8-byte aligned since this struct will be 8-byte aligned
    len: u8,
    pub lamports: u64,
    pub rent_epoch: u64,
    pub owner: Pubkey,
    pub executable: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SmallAccountTryNewParams<'a> {
    pub data: &'a [u8],
    pub lamports: u64,
    pub rent_epoch: u64,
    pub owner: Pubkey,
    pub executable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataTooLong;

impl Display for DataTooLong {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Account data too long")
    }
}

impl Error for DataTooLong {}

impl SmallAccount {
    pub fn try_new(
        SmallAccountTryNewParams {
            data,
            lamports,
            rent_epoch,
            owner,
            executable,
        }: SmallAccountTryNewParams,
    ) -> Result<Self, DataTooLong> {
        let len = data.len();
        if len > SMALL_ACCOUNT_DATA_MAX_LEN_USIZE {
            return Err(DataTooLong);
        }
        let mut res = Self {
            data: Default::default(),
            len: Default::default(),
            lamports,
            rent_epoch,
            owner,
            executable,
        };
        res.data.copy_from_slice(data);
        res.len = len.try_into().unwrap();
        Ok(res)
    }

    pub fn data_slice(&self) -> &[u8] {
        &self.data[..self.len.into()]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SmallAccountDataRef<'a>(pub &'a [u8]);

impl<'a> Deref for SmallAccountDataRef<'a> {
    type Target = &'a [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ReadonlyAccountData for SmallAccount {
    type SliceDeref<'s> = &'s[u8]
    where
        Self: 's;

    type DataDeref<'d> = SmallAccountDataRef<'d>
    where
        Self: 'd;

    fn data(&self) -> Self::DataDeref<'_> {
        SmallAccountDataRef(self.data_slice())
    }
}

impl ReadonlyAccountIsExecutable for SmallAccount {
    fn executable(&self) -> bool {
        self.executable
    }
}

impl ReadonlyAccountLamports for SmallAccount {
    fn lamports(&self) -> u64 {
        self.lamports
    }
}

impl ReadonlyAccountOwner for SmallAccount {
    fn owner(&self) -> &Pubkey {
        &self.owner
    }
}

impl ReadonlyAccountRentEpoch for SmallAccount {
    fn rent_epoch(&self) -> u64 {
        self.rent_epoch
    }
}

impl PartialEq for SmallAccount {
    fn eq(&self, other: &Self) -> bool {
        self.data_slice() == other.data_slice()
            && self.lamports == other.lamports
            && self.rent_epoch == other.rent_epoch
            && self.owner == other.owner
            && self.executable == other.executable
    }
}

impl Eq for SmallAccount {}

impl Hash for SmallAccount {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data_slice().hash(state);
        self.lamports.hash(state);
        self.rent_epoch.hash(state);
        self.owner.hash(state);
        self.executable.hash(state);
    }
}
