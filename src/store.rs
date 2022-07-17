use handle_errors::Error;
use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};

use crate::types::{
    answers::{Answer, AnswerId, NewAnswer},
    questions::{NewQuestion, Question, QuestionId},
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

                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, Error> {
        let query_result = sqlx::query(
            "
            INSERT INTO questions (title, content, tags) 
            VALUES ($1, $2, $3) RETURNING id, title, content, tags;
            ",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
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

                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn get_question(&self, question_id: u32) -> Result<Question, Error> {
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

                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn update_question(
        &self,
        question: Question,
        question_id: u32,
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

                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn delete_question(&self, question_id: u32) -> Result<bool, Error> {
        let query_result = sqlx::query("DELETE FROM questions WHERE id = $1;")
            .bind(question_id)
            .execute(&self.connection)
            .await;

        match query_result {
            Ok(_) => Ok(true),
            Err(err) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", err);

                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn add_answer(&self, new_answer: NewAnswer) -> Result<Answer, Error> {
        let query_result = sqlx::query(
            "
            INSERT INTO answers (content, corresponding_question) 
            VALUES ($1, $2) RETURNING id, content, corresponding_question;
            ",
        )
        .bind(new_answer.content)
        .bind(new_answer.question_id.0)
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

                Err(Error::DatabaseQueryError)
            }
        }
    }
}
