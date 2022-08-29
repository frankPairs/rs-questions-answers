use handle_errors::Error;

use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};

use crate::{
    crypt::hash_password,
    types::{
        account::{Account, AccountId, NewAccount},
        answers::{Answer, AnswerId, NewAnswer},
        questions::{NewQuestion, Question, QuestionId},
    },
};

const DB_MAX_CONNECTIONS: u32 = 5;

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(DB_MAX_CONNECTIONS)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(_) => panic!("Could not establsh a database connection"),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(
        &self,
        limit: Option<u32>,
        offset: u32,
    ) -> Result<Vec<Question>, Error> {
        let query_result = sqlx::query("SELECT * FROM questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await;

        match query_result {
            Ok(questions) => Ok(questions),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "Get Questions Error: {:?}", err);

                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, Error> {
        let query_result = sqlx::query(
            "
            INSERT INTO questions (title, content, tags, account_id) 
            VALUES ($1, $2, $3, $4) RETURNING id, title, content, tags;
            ",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .bind(new_question.account_id.0)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await;

        match query_result {
            Ok(question) => Ok(question),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);

                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub async fn get_question(&self, question_id: i32) -> Result<Question, Error> {
        let query_result = sqlx::query("SELECT * FROM questions WHERE id = $1;")
            .bind(question_id)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_one(&self.connection)
            .await;

        match query_result {
            Ok(question) => Ok(question),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);

                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub async fn update_question(
        &self,
        question: Question,
        question_id: i32,
    ) -> Result<Question, Error> {
        let query_result = sqlx::query(
            "
            UPDATE questions 
            SET title = $1, content = $2, tags = $3 
            WHERE id = $4
            RETURNING id, title, content, tags;
            ",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await;

        match query_result {
            Ok(question) => Ok(question),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "Update Question Error: {:?}", err);

                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub async fn delete_question(&self, question_id: i32) -> Result<bool, Error> {
        let query_result = sqlx::query("DELETE FROM questions WHERE id = $1;")
            .bind(question_id)
            .execute(&self.connection)
            .await;

        match query_result {
            Ok(_) => Ok(true),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);

                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub async fn add_answer(&self, new_answer: NewAnswer) -> Result<Answer, Error> {
        let query_result = sqlx::query(
            "
            INSERT INTO answers (content, corresponding_question, account_id) 
            VALUES ($1, $2, $3) RETURNING id, content, corresponding_question;
            ",
        )
        .bind(new_answer.content)
        .bind(new_answer.question_id.0)
        .bind(new_answer.account_id.0)
        .map(|row: PgRow| Answer {
            id: AnswerId(row.get("id")),
            content: row.get("content"),
            question_id: QuestionId(row.get("corresponding_question")),
        })
        .fetch_one(&self.connection)
        .await;

        match query_result {
            Ok(answer) => Ok(answer),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);

                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub async fn add_account(&self, new_account: NewAccount) -> Result<Account, Error> {
        let hashed_password = hash_password(new_account.password.as_bytes());
        let query_result = sqlx::query(
            "
            INSERT INTO accounts (email, password) 
            VALUES ($1, $2) RETURNING id, email, password;
            ",
        )
        .bind(new_account.email)
        .bind(hashed_password)
        .map(|row: PgRow| Account {
            id: AccountId(row.get("id")),
            email: row.get("email"),
            password: row.get("password"),
        })
        .fetch_one(&self.connection)
        .await;

        match query_result {
            Ok(account) => Ok(account),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);

                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub async fn get_account(&self, email: String) -> Result<Account, Error> {
        let query_result = sqlx::query("SELECT * FROM accounts WHERE email = $1;")
            .bind(email)
            .map(|row: PgRow| Account {
                id: AccountId(row.get("id")),
                email: row.get("email"),
                password: row.get("password"),
            })
            .fetch_one(&self.connection)
            .await;

        match query_result {
            Ok(account) => Ok(account),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);

                Err(Error::DatabaseQueryError(err))
            }
        }
    }

    pub async fn is_question_owner(
        &self,
        question_id: i32,
        account_id: i32,
    ) -> Result<bool, Error> {
        let query_result =
            sqlx::query("SELECT * FROM questions WHERE id = $1 AND account_id = $2;")
                .bind(question_id)
                .bind(account_id)
                .fetch_optional(&self.connection)
                .await;

        match query_result {
            Ok(question) => Ok(question.is_some()),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);

                Err(Error::DatabaseQueryError(err))
            }
        }
    }
}
