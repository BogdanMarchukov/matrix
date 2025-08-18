use tonic::{async_trait, transport::Channel, Request, Response, Status};

use crate::compute::{
    matrix_schema_client::MatrixSchemaClient, MatrixSchemaRequest, MatrixSchemaResponse,
};

use super::traits::MatrixSchemaSvc;

#[derive(Clone)]
pub struct MatrixSchemaSvcWrapper {
    inner: MatrixSchemaClient<Channel>,
}

impl MatrixSchemaSvcWrapper {
    pub fn new(inner: MatrixSchemaClient<Channel>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl MatrixSchemaSvc for MatrixSchemaSvcWrapper {
    async fn calc_matrix_schema(
        &self,
        req: Request<MatrixSchemaRequest>,
    ) -> Result<Response<MatrixSchemaResponse>, Status> {
        let mut c = self.inner.clone();
        c.calc_matrix_schema(req).await
    }
}
