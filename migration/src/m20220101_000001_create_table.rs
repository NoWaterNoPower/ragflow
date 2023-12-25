use sea_orm_migration::{ prelude::*, sea_orm::Statement };
use chrono::{ FixedOffset, Utc };

fn now() -> chrono::DateTime<FixedOffset> {
    Utc::now().with_timezone(&FixedOffset::east_opt(3600 * 8).unwrap())
}
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(UserInfo::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(UserInfo::Uid)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(UserInfo::Email).string().not_null())
                .col(ColumnDef::new(UserInfo::Nickname).string().not_null())
                .col(ColumnDef::new(UserInfo::AvatarBase64).string())
                .col(ColumnDef::new(UserInfo::ColorScheme).string().default("dark"))
                .col(ColumnDef::new(UserInfo::ListStyle).string().default("list"))
                .col(ColumnDef::new(UserInfo::Language).string().default("chinese"))
                .col(ColumnDef::new(UserInfo::Password).string().not_null())
                .col(
                    ColumnDef::new(UserInfo::LastLoginAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                )
                .col(
                    ColumnDef::new(UserInfo::CreatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(
                    ColumnDef::new(UserInfo::UpdatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(ColumnDef::new(UserInfo::IsDeleted).boolean().default(false))
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(TagInfo::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(TagInfo::Tid)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(TagInfo::Uid).big_integer().not_null())
                .col(ColumnDef::new(TagInfo::TagName).string().not_null())
                .col(ColumnDef::new(TagInfo::Regx).string())
                .col(ColumnDef::new(TagInfo::Color).tiny_unsigned().default(1))
                .col(ColumnDef::new(TagInfo::Icon).tiny_unsigned().default(1))
                .col(ColumnDef::new(TagInfo::FolderId).big_integer())
                .col(
                    ColumnDef::new(TagInfo::CreatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(
                    ColumnDef::new(TagInfo::UpdatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(ColumnDef::new(TagInfo::IsDeleted).boolean().default(false))
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(Tag2Doc::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Tag2Doc::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(Tag2Doc::TagId).big_integer())
                .col(ColumnDef::new(Tag2Doc::Did).big_integer())
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(Kb2Doc::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Kb2Doc::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(Kb2Doc::KbId).big_integer())
                .col(ColumnDef::new(Kb2Doc::Did).big_integer())
                .col(ColumnDef::new(Kb2Doc::KbProgress).float().default(0))
                .col(ColumnDef::new(Kb2Doc::KbProgressMsg).string().default(""))
                .col(
                    ColumnDef::new(Kb2Doc::UpdatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(ColumnDef::new(Kb2Doc::IsDeleted).boolean().default(false))
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(Dialog2Kb::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Dialog2Kb::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(Dialog2Kb::DialogId).big_integer())
                .col(ColumnDef::new(Dialog2Kb::KbId).big_integer())
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(Doc2Doc::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(Doc2Doc::Id)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(Doc2Doc::ParentId).big_integer())
                .col(ColumnDef::new(Doc2Doc::Did).big_integer())
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(KbInfo::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(KbInfo::KbId)
                        .big_integer()
                        .auto_increment()
                        .not_null()
                        .primary_key()
                )
                .col(ColumnDef::new(KbInfo::Uid).big_integer().not_null())
                .col(ColumnDef::new(KbInfo::KbName).string().not_null())
                .col(ColumnDef::new(KbInfo::Icon).tiny_unsigned().default(1))
                .col(
                    ColumnDef::new(KbInfo::CreatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(
                    ColumnDef::new(KbInfo::UpdatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(ColumnDef::new(KbInfo::IsDeleted).boolean().default(false))
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(DocInfo::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(DocInfo::Did)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(DocInfo::Uid).big_integer().not_null())
                .col(ColumnDef::new(DocInfo::DocName).string().not_null())
                .col(ColumnDef::new(DocInfo::Location).string().not_null())
                .col(ColumnDef::new(DocInfo::Size).big_integer().not_null())
                .col(ColumnDef::new(DocInfo::Type).string().not_null())
                .comment("doc|folder")
                .col(
                    ColumnDef::new(DocInfo::CreatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(
                    ColumnDef::new(DocInfo::UpdatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(ColumnDef::new(DocInfo::IsDeleted).boolean().default(false))
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(DialogInfo::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(DialogInfo::DialogId)
                        .big_integer()
                        .not_null()
                        .auto_increment()
                        .primary_key()
                )
                .col(ColumnDef::new(DialogInfo::Uid).big_integer().not_null())
                .col(ColumnDef::new(DialogInfo::KbId).big_integer().not_null())
                .col(ColumnDef::new(DialogInfo::DialogName).string().not_null())
                .col(ColumnDef::new(DialogInfo::History).string().comment("json"))
                .col(
                    ColumnDef::new(DialogInfo::CreatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(
                    ColumnDef::new(DialogInfo::UpdatedAt)
                        .timestamp_with_time_zone()
                        .default(Expr::current_timestamp())
                        .not_null()
                )
                .col(ColumnDef::new(DialogInfo::IsDeleted).boolean().default(false))
                .to_owned()
        ).await?;

        let tm = now();
        let root_insert = Query::insert()
            .into_table(UserInfo::Table)
            .columns([UserInfo::Email, UserInfo::Nickname, UserInfo::Password])
            .values_panic(["kai.hu@infiniflow.org".into(), "root".into(), "123456".into()])
            .to_owned();

        let doc_insert = Query::insert()
            .into_table(DocInfo::Table)
            .columns([
                DocInfo::Uid,
                DocInfo::DocName,
                DocInfo::Size,
                DocInfo::Type,
                DocInfo::Location,
            ])
            .values_panic([(1).into(), "/".into(), (0).into(), "folder".into(), "".into()])
            .to_owned();

        let tag_insert = Query::insert()
            .into_table(TagInfo::Table)
            .columns([TagInfo::Uid, TagInfo::TagName, TagInfo::Regx, TagInfo::Color, TagInfo::Icon])
            .values_panic([
                (1).into(),
                "视频".into(),
                ".*\\.(mpg|mpeg|avi|rm|rmvb|mov|wmv|asf|dat|asx|wvx|mpe|mpa)".into(),
                (1).into(),
                (1).into(),
            ])
            .values_panic([
                (1).into(),
                "图片".into(),
                ".*\\.(png|tif|gif|pcx|tga|exif|fpx|svg|psd|cdr|pcd|dxf|ufo|eps|ai|raw|WMF|webp|avif|apng)".into(),
                (2).into(),
                (2).into(),
            ])
            .values_panic([
                (1).into(),
                "音乐".into(),
                ".*\\.(WAV|FLAC|APE|ALAC|WavPack|WV|MP3|AAC|Ogg|Vorbis|Opus)".into(),
                (3).into(),
                (3).into(),
            ])
            .values_panic([
                (1).into(),
                "文档".into(),
                ".*\\.(pdf|doc|ppt|yml|xml|htm|json|csv|txt|ini|xsl|wps|rtf|hlp)".into(),
                (3).into(),
                (3).into(),
            ])
            .to_owned();

        manager.exec_stmt(root_insert).await?;
        manager.exec_stmt(doc_insert).await?;
        manager.exec_stmt(tag_insert).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(UserInfo::Table).to_owned()).await?;

        manager.drop_table(Table::drop().table(TagInfo::Table).to_owned()).await?;

        manager.drop_table(Table::drop().table(Tag2Doc::Table).to_owned()).await?;

        manager.drop_table(Table::drop().table(Kb2Doc::Table).to_owned()).await?;

        manager.drop_table(Table::drop().table(Dialog2Kb::Table).to_owned()).await?;

        manager.drop_table(Table::drop().table(Doc2Doc::Table).to_owned()).await?;

        manager.drop_table(Table::drop().table(KbInfo::Table).to_owned()).await?;

        manager.drop_table(Table::drop().table(DocInfo::Table).to_owned()).await?;

        manager.drop_table(Table::drop().table(DialogInfo::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum UserInfo {
    Table,
    Uid,
    Email,
    Nickname,
    AvatarBase64,
    ColorScheme,
    ListStyle,
    Language,
    Password,
    LastLoginAt,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}

#[derive(DeriveIden)]
enum TagInfo {
    Table,
    Tid,
    Uid,
    TagName,
    Regx,
    Color,
    Icon,
    FolderId,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}

#[derive(DeriveIden)]
enum Tag2Doc {
    Table,
    Id,
    TagId,
    Did,
}

#[derive(DeriveIden)]
enum Kb2Doc {
    Table,
    Id,
    KbId,
    Did,
    KbProgress,
    KbProgressMsg,
    UpdatedAt,
    IsDeleted,
}

#[derive(DeriveIden)]
enum Dialog2Kb {
    Table,
    Id,
    DialogId,
    KbId,
}

#[derive(DeriveIden)]
enum Doc2Doc {
    Table,
    Id,
    ParentId,
    Did,
}

#[derive(DeriveIden)]
enum KbInfo {
    Table,
    KbId,
    Uid,
    KbName,
    Icon,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}

#[derive(DeriveIden)]
enum DocInfo {
    Table,
    Did,
    Uid,
    DocName,
    Location,
    Size,
    Type,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}

#[derive(DeriveIden)]
enum DialogInfo {
    Table,
    Uid,
    KbId,
    DialogId,
    DialogName,
    History,
    CreatedAt,
    UpdatedAt,
    IsDeleted,
}