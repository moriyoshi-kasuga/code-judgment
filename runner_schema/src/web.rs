use crate::{Language, memory::Memory, state::RunnerState, time::MsTime};

tonic::include_proto!("runner");

#[derive(Debug, thiserror::Error)]
#[error("Failed to convert to/from gRPC")]
pub struct GrpcConvertError {
    #[source]
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
    field: String,
}

#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct RunnerRequest {
    pub lang: Language,
    pub code: String,
    pub ms_time_limit: MsTime,
    pub memory_limit: Memory,
    pub stdin: String,
}

#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct RunnerResponse {
    pub state: RunnerState,
}

impl RunnerRequest {
    pub fn into_grpc(self) -> GrpcRunnerRequest {
        GrpcRunnerRequest {
            lang: self.lang.into(),
            code: self.code,
            ms_time_limit: self.ms_time_limit.as_ms(),
            memory_limit: self.memory_limit.as_bytes(),
            stdin: self.stdin,
        }
    }
}

impl RunnerResponse {
    pub fn try_into_grpc(self) -> Result<GrpcRunnerResponse, GrpcConvertError> {
        Ok(GrpcRunnerResponse {
            state: match self.state.to_json() {
                Ok(state) => state,
                Err(err) => {
                    return Err(GrpcConvertError {
                        source: Box::new(err),
                        field: "state".to_string(),
                    });
                }
            },
        })
    }
}

impl GrpcRunnerRequest {
    pub fn try_into_web(self) -> Result<RunnerRequest, GrpcConvertError> {
        Ok(RunnerRequest {
            lang: match Language::try_from(self.lang) {
                Ok(lang) => lang,
                Err(err) => {
                    return Err(GrpcConvertError {
                        source: Box::new(err),
                        field: "lang".to_string(),
                    });
                }
            },
            code: self.code,
            ms_time_limit: MsTime::new_ms(self.ms_time_limit),
            memory_limit: Memory::new_bytes(self.memory_limit),
            stdin: self.stdin,
        })
    }
}

impl GrpcRunnerResponse {
    pub fn try_from_web(self) -> Result<RunnerResponse, GrpcConvertError> {
        Ok(RunnerResponse {
            state: match RunnerState::from_json(&self.state) {
                Ok(state) => state,
                Err(err) => {
                    return Err(GrpcConvertError {
                        source: Box::new(err),
                        field: "state".to_string(),
                    });
                }
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_request_conversion() {
        let request = RunnerRequest {
            lang: Language::Rust1_82,
            code: "fn main() { println!(\"Hello, world!\"); }".to_string(),
            ms_time_limit: MsTime::new_ms(1000),
            memory_limit: Memory::new_bytes(1024 * 1024),
            stdin: "input".to_string(),
        };

        let grpc_request = request.clone().into_grpc();
        let converted_request = grpc_request.try_into_web().unwrap();

        assert_eq!(request, converted_request);
    }

    #[test]
    fn test_grpc_response_conversion() {
        let response = RunnerResponse {
            state: RunnerState::Success {
                stdout: "Hello, world!".to_string(),
                max_memory_usage: Memory::new_bytes(1024 * 1024),
                ms_time_elapsed: MsTime::new_ms(500),
            },
        };

        let grpc_response = response.clone().try_into_grpc().unwrap();
        let converted_response = grpc_response.try_from_web().unwrap();

        assert_eq!(response, converted_response);
    }

    #[test]
    fn test_grpc_request_error_handling() {
        let grpc_request = GrpcRunnerRequest {
            lang: 999, // Invalid language
            code: "fn main() { println!(\"Hello, world!\"); }".to_string(),
            ms_time_limit: 1000,
            memory_limit: 1024 * 1024,
            stdin: "input".to_string(),
        };

        let result = grpc_request.try_into_web();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().field, "lang");
    }

    #[test]
    fn test_grpc_response_error_handling() {
        let grpc_response = GrpcRunnerResponse {
            state: "invalid_json".to_string(), // Invalid JSON
        };

        let result = grpc_response.try_from_web();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().field, "state");
    }
}
