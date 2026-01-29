use time::OffsetDateTime;
use uuid::Uuid;

use crate::pb::explanation::{ExplanationItem, GetExplanationByIdResponse};

/// 数据库中查到的数据
#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExplanationHu {
    pub id: i32,
    pub uuid: Uuid,
    pub explanation: serde_json::Value,
    pub summary: Vec<String>,
    #[serde(with = "time::serde::rfc3339")]
    #[serde(rename = "createdAt")]
    pub created_at: OffsetDateTime,
}
// impl serde::Serialize for ExplanationHu {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut state = serializer.serialize_struct("ExplanationHu", 5)?;

//         // 按照你想要的顺序添加字段
//         state.serialize_field("id", &self.id)?;
//         state.serialize_field("uuid", &self.uuid)?;
//         state.serialize_field("explanation", &self.explanation)?;
//         state.serialize_field("summary", &self.summary)?;
//         state.serialize_field("createdAt", &self.created_at)?;

//         state.end()
//     }
// }
/// 将 ExplanationHu 转换为 GetExplanationByIdResponse
impl From<ExplanationHu> for GetExplanationByIdResponse {
    fn from(value: ExplanationHu) -> Self {
        // 将 serde_json::Value 数组转换为 Vec<ExplanationItem>
        let explanation = match value.explanation {
            serde_json::Value::Array(arr) => {
                arr.into_iter()
                    .filter_map(|item| {
                        if let serde_json::Value::Object(obj) = item {
                            // 获取第一个（也是唯一一个）键值对
                            obj.into_iter().next().map(|(key, value)| ExplanationItem {
                                key,
                                value: value
                                    .as_str()
                                    .map(|s| s.to_string())
                                    .unwrap_or_else(|| value.to_string()),
                            })
                        } else {
                            None
                        }
                    })
                    .collect()
            }
            _ => Vec::new(), // 如果不是数组，返回空向量
        };

        // 将 OffsetDateTime 转换为 prost_types::Timestamp
        let created_at = if value.created_at == OffsetDateTime::UNIX_EPOCH {
            None
        } else {
            Some(prost_types::Timestamp {
                seconds: value.created_at.unix_timestamp(),
                nanos: value.created_at.nanosecond() as i32,
            })
        };

        GetExplanationByIdResponse {
            treatise_id: value.id,
            uuid: value.uuid.to_string(),
            explanation,
            summary: value.summary,
            created_at,
        }
    }
}
/// 将 GetExplanationByIdResponse 转换为 ExplanationHu
impl From<GetExplanationByIdResponse> for ExplanationHu {
    fn from(value: GetExplanationByIdResponse) -> Self {
        // 将 Vec<ExplanationItem> 转换为 serde_json::Value 数组
        let explanation_value = serde_json::Value::Array(
            value
                .explanation
                .into_iter()
                .map(|item| {
                    let mut map = serde_json::Map::new();
                    map.insert(item.key, serde_json::Value::String(item.value));
                    serde_json::Value::Object(map)
                })
                .collect(),
        );

        // 将 prost_types::Timestamp 转换为 OffsetDateTime
        let created_at = value
            .created_at
            .map(|ts| {
                // 使用 time crate 创建 OffsetDateTime
                let datetime = time::OffsetDateTime::from_unix_timestamp(ts.seconds)
                    .unwrap_or(time::OffsetDateTime::UNIX_EPOCH);
                datetime
                    .replace_nanosecond(ts.nanos as u32)
                    .unwrap_or(time::OffsetDateTime::UNIX_EPOCH)
            })
            .unwrap_or(time::OffsetDateTime::UNIX_EPOCH);

        ExplanationHu {
            id: value.treatise_id,
            uuid: Uuid::parse_str(&value.uuid).unwrap_or_else(|_| Uuid::nil()), // 如果解析失败，使用nil UUID
            explanation: explanation_value,
            summary: value.summary,
            created_at,
        }
    }
}
