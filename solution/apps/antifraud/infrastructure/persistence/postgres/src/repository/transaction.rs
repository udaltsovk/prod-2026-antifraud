use application::repository::transaction::TransactionRepository;
use domain::{
    pagination::{Pagination, time_based::TimeBasedPagination},
    transaction::{Transaction, filter::TransactionFilter},
};
use lib::{
    anyhow::Result,
    async_trait,
    domain::{DomainType, Id},
    instrument_all,
    tap::Pipe as _,
    uuid::Uuid,
};
use serde_json::Value;
use sqlx::{
    Acquire as _, Executor, Postgres, query_file_as, query_file_scalar,
};

use crate::{
    entity::transaction::{
        StoredTransaction, channel::StoredTransactionChannel,
        location::StoredTransactionLocation, verdict::StoredTransactionVerdict,
    },
    repository::PostgresRepositoryImpl,
};

#[async_trait]
#[instrument_all]
impl TransactionRepository for PostgresRepositoryImpl<Transaction> {
    async fn save(&self, source: Transaction) -> Result<Transaction> {
        let mut connection = self.pool.get().await?;

        Self::save_transaction(&mut *connection, source).await
    }

    async fn batch_save(
        &self,
        sources: Vec<Transaction>,
    ) -> Result<Vec<Transaction>> {
        let mut transactions = Vec::new();

        let mut connection = self.pool.get().await?;
        let mut transaction = connection.begin().await?;

        for source in sources {
            Self::save_transaction(&mut *transaction, source)
                .await
                .map(|transaction| transactions.push(transaction))?;
        }

        transaction.commit().await?;

        Ok(transactions)
    }

    async fn find_by_id(
        &self,
        transaction_id: Id<Transaction>,
    ) -> Result<Option<Transaction>> {
        let transaction_id = transaction_id.value;

        let mut connection = self.pool.get().await?;

        query_file_as!(
            StoredTransaction,
            "sql/transaction/find_by_id.sql",
            transaction_id
        )
        .fetch_optional(&mut *connection)
        .await?
        .map(Transaction::from)
        .pipe(Ok)
    }

    async fn list(
        &self,
        TransactionFilter {
            requester_id,
            status,
            time_based_pagination:
                TimeBasedPagination {
                    from,
                    to,
                },
            pagination:
                Pagination {
                    limit,
                    offset,
                },
        }: TransactionFilter,
    ) -> Result<Vec<Transaction>> {
        let requester_id = requester_id.map(Uuid::from);
        let verdict = status.map(StoredTransactionVerdict::from);

        let mut connection = self.pool.get().await?;

        query_file_as!(
            StoredTransaction,
            "sql/transaction/list.sql",
            requester_id,
            verdict as _,
            from,
            to,
            limit,
            offset
        )
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .map(Transaction::from)
        .collect::<Vec<_>>()
        .pipe(Ok)
    }

    async fn count(
        &self,
        TransactionFilter {
            requester_id,
            status,
            time_based_pagination:
                TimeBasedPagination {
                    from,
                    to,
                },
            ..
        }: TransactionFilter,
    ) -> Result<i64> {
        let requester_id = requester_id.map(Uuid::from);
        let verdict = status.map(StoredTransactionVerdict::from);

        let mut connection = self.pool.get().await?;

        query_file_scalar!(
            "sql/transaction/count.sql",
            requester_id,
            verdict as _,
            from,
            to
        )
        .fetch_one(&mut *connection)
        .await?
        .unwrap_or_default()
        .pipe(Ok)
    }
}

#[instrument_all]
impl PostgresRepositoryImpl<Transaction> {
    async fn save_transaction<'c, E>(
        executor: E,
        source: Transaction,
    ) -> Result<Transaction>
    where
        E: Executor<'c, Database = Postgres>,
    {
        let transaction_id = source.id.value;
        let user_id = source.user_id.into_inner();
        let amount = source.amount.into_inner();
        let currency = source.currency.into_inner();
        let verdict: StoredTransactionVerdict = source.status.into();
        let merchant_id = source.merchant_id.map(DomainType::into_inner);
        let merchant_category_code =
            source.merchant_category_code.map(DomainType::into_inner);
        let specified_timestamp = source.timestamp.into_inner();
        let ip_address = source.ip_address.map(DomainType::into_inner);
        let device_id = source.device_id.map(DomainType::into_inner);
        let channel = source.channel.map(StoredTransactionChannel::from);
        let location: StoredTransactionLocation = source.location.into();
        let metadata = source.metadata.map(Value::from);

        query_file_as!(
            StoredTransaction,
            "sql/transaction/create.sql",
            transaction_id,
            user_id,
            amount,
            currency,
            verdict as _,
            merchant_id,
            merchant_category_code,
            specified_timestamp,
            ip_address as _,
            device_id,
            channel as _,
            location as _,
            metadata
        )
        .fetch_one(executor)
        .await
        .map_err(Into::into)
        .map(Transaction::from)
    }
}
