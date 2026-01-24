use domain::{
    transaction::{
        CreateTransaction, Transaction, decision::TransactionDecision,
        pagination::TransactionPagination,
    },
    user::{User, role::UserRole},
};
use lib::{
    async_trait,
    domain::{
        Id,
        validation::{ExternalInput, error::ValidationResult},
    },
    uuid::Uuid,
};

use crate::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::transaction::error::TransactionUseCaseResult,
};

pub mod error;
pub mod implementation;

#[async_trait]
pub trait TransactionUseCase<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn create(
        &self,
        creator: (Id<User>, UserRole),
        input: (ValidationResult<CreateTransaction>, ExternalInput<Uuid>),
    ) -> TransactionUseCaseResult<R, S, TransactionDecision>;

    async fn bulk_create(
        &self,
        creator: (Id<User>, UserRole),
        input: ValidationResult<CreateTransaction>,
    ) -> Vec<(i64, TransactionUseCaseResult<R, S, TransactionDecision>)>;

    async fn find_by_id(
        &self,
        requester: (Id<User>, UserRole),
        transaction_id: Id<Transaction>,
    ) -> TransactionUseCaseResult<R, S, Option<TransactionDecision>>;

    async fn get_by_id(
        &self,
        requester: (Id<User>, UserRole),
        transaction_id: Id<Transaction>,
    ) -> TransactionUseCaseResult<R, S, TransactionDecision>;

    async fn list(
        &self,
        requester: (Id<User>, UserRole),
        input: (ValidationResult<TransactionPagination>, ExternalInput<Uuid>),
    ) -> TransactionUseCaseResult<R, S, (Vec<Transaction>, u64)>;
}
