use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use shared::structures::{
    ids::PresetId,
    user_settings::prelude::{Emoji, PresenceIcon, PresenceKind, PresencePreset, PresenceTimer},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "presence_presets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub user_id: String,
    pub label: String,
    pub icon_kind: String,
    pub icon_value: String,
    pub timer_kind: String,
    pub timer_seconds: Option<i64>,
    pub preset_kind: String,
    pub process_name: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for PresencePreset {
    fn from(model: Model) -> Self {
        PresencePreset {
            id: PresetId(model.id),
            label: model.label,
            icon: match model.icon_kind.as_str() {
                "emoji" => PresenceIcon::Emoji {
                    emoji: Emoji::Unicode {
                        value: model.icon_value,
                    },
                },
                "app" => PresenceIcon::AppIcon {
                    process_name: model.icon_value,
                },
                _ => PresenceIcon::CustomUpload {
                    path_url: model.icon_value,
                },
            },
            timer: match model.timer_kind.as_str() {
                "elapsed" => PresenceTimer::Elapsed,
                "countdown" => PresenceTimer::Countdown {
                    seconds: model.timer_seconds.unwrap_or(0) as u64,
                },
                _ => PresenceTimer::None,
            },
            kind: match model.preset_kind.as_str() {
                "app_linked" => PresenceKind::AppLinked {
                    process_name: model.process_name.unwrap_or_default(),
                },
                _ => PresenceKind::Fixed,
            },
        }
    }
}
