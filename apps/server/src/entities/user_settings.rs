use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use shared::structures::{
    user::UserSettings,
    user_settings::{
        prelude::{Locale, NotificationSettings, UISettings},
        ui::{DarkMode, Rounding, Spacing, Theme},
    },
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "user_settings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub user_id: String,
    pub locale: String,
    pub developer_mode: bool,
    pub notifications_active: bool,
    pub theme_dark_mode: String,
    pub theme_color: String,
    pub theme_rounding: String,
    pub theme_spacing: String,
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

impl From<Model> for UserSettings {
    fn from(model: Model) -> Self {
        UserSettings {
            developer_mode: model.developer_mode,
            locale: match model.locale.as_str() {
                "pt-BR" => Locale::PtBr,
                _ => Locale::EnUS,
            },
            notifications: NotificationSettings {
                active: model.notifications_active,
            },
            ui: UISettings {
                dark_mode: match model.theme_dark_mode.as_str() {
                    "light" => DarkMode::Light,
                    "dark" => DarkMode::Dark,
                    _ => DarkMode::System,
                },
                theme: match model.theme_color.as_str() {
                    "strawberry" => Theme::Strawberry,
                    "blueberry" => Theme::Blueberry,
                    _ => Theme::Default,
                },
                rounding: match model.theme_rounding.as_str() {
                    "none" => Rounding::None,
                    "full" => Rounding::Full,
                    _ => Rounding::Default,
                },
                spacing: match model.theme_spacing.as_str() {
                    "compact" => Spacing::Compact,
                    "comfortable" => Spacing::Comfortable,
                    _ => Spacing::Default,
                },
            },
            presence_presets: vec![],
        }
    }
}
