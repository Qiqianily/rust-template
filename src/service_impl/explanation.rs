use std::{ops::Deref, sync::Arc};

use crate::{
    pb::explanation::{
        GetExplanationByIdRequest, GetExplanationByIdResponse,
        explanation_hu_service_server::ExplanationHuService,
    },
    service_impl::model::ExplanationHu,
};
use sqlx::PgPool;

use tonic::{Request, Response, Status};

/// 内部数据状态
#[derive(Debug, Clone)]
pub struct AppStateInner {
    pub pool: &'static PgPool,
}
/// 实现解引用操作
impl Deref for ExplanationHuServiceImpl {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// 实现 GreeterService trait
#[derive(Debug)]
pub struct ExplanationHuServiceImpl {
    // 这里面可以放数据库连接池
    pub inner: Arc<AppStateInner>,
}
impl ExplanationHuServiceImpl {
    pub fn new(pool: &'static PgPool) -> Self {
        Self {
            inner: Arc::new(AppStateInner { pool }),
        }
    }
}
#[tonic::async_trait]
impl ExplanationHuService for ExplanationHuServiceImpl {
    async fn get_explanation_by_id(
        &self,
        request: Request<GetExplanationByIdRequest>,
    ) -> Result<Response<GetExplanationByIdResponse>, Status> {
        let id = request.into_inner().id;
        // 查询数据库
        let explanation: ExplanationHu = sqlx::query_as(
                "SELECT treatise_id as id, uuid, explanation, summary, created_at FROM explain_hu WHERE treatise_id = $1",
            )
            .bind(id)
            .fetch_one(self.inner.pool)
            .await.map_err(|e| Status::internal(e.to_string()))?;
        // 返回结果
        Ok(Response::new(explanation.into()))
    }
}
