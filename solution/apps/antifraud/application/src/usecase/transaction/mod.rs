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
        validation::{ExternalInput, error::ValidationResultWithFields},
    },
    uuid::Uuid,
};

use crate::usecase::transaction::error::TransactionUseCaseResult;

pub mod error;
pub mod implementation;

#[async_trait]
pub trait TransactionUseCase {
    async fn create(
        &self,
        creator: (Id<User>, UserRole),
        input: (
            ValidationResultWithFields<CreateTransaction>,
            ExternalInput<Uuid>,
        ),
    ) -> TransactionUseCaseResult<TransactionDecision>;

    async fn bulk_create(
        &self,
        creator: (Id<User>, UserRole),
        input: Vec<(
            ValidationResultWithFields<CreateTransaction>,
            ExternalInput<Uuid>,
        )>,
    ) -> TransactionUseCaseResult<
        Vec<(usize, TransactionUseCaseResult<TransactionDecision>)>,
    >;

    async fn find_by_id(
        &self,
        requester: (Id<User>, UserRole),
        transaction_id: Id<Transaction>,
    ) -> TransactionUseCaseResult<Option<TransactionDecision>>;

    async fn get_by_id(
        &self,
        requester: (Id<User>, UserRole),
        transaction_id: Id<Transaction>,
    ) -> TransactionUseCaseResult<TransactionDecision>;

    async fn list(
        &self,
        requester: (Id<User>, UserRole),
        input: (
            ValidationResultWithFields<TransactionPagination>,
            ExternalInput<Uuid>,
        ),
    ) -> TransactionUseCaseResult<(Vec<Transaction>, u64)>;
}
