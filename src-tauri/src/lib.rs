// импоорт типов необходимых для migrations
use tauri_plugin_sql::{Migration, MigrationKind};

// аннотация необходимая tauri для мобильных платформ
#[cfg_attr(mobile, tauri::mobile_entry_point)]

// главная функция для запуска приложения
pub fn run() {
    // создание списков миграций
    let migrations = vec![
        // описание
        Migration {
            version: 1,
            description: "create_message_table",

            // берем sql запрос из файла
            sql: include_str!("../migrations/0001_initial.sql"),

            // up - база сдвинется вперед
            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:messanger.db", migrations)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        // запускаем приложение
        .run(tauri::generate_context!())
        // если запуск с ошибкой, то сообщаем
        .expect("error while running tauri application");
}
