use std::collections::HashMap;

pub const EXIT_COMMAND: &str = ".съебать";

pub struct Commands {
    pub dict: Vec<(String, String)>,
}

impl Commands {
    pub fn new() -> Commands {
        let dict = vec![
            (String::from("сунуть"), String::from("insert")),
            (String::from("ебнуть"), String::from("delete")),
            (String::from("объебать"), String::from("update")),
            (String::from("захуярить"), String::from("create")),
            (String::from("естьче?"), String::from("select")),
            (String::from("аеслинайду?"), String::from("where")),
            (String::from("здесь"), String::from("from")),
            (String::from("высрать по"), String::from("group by")),
            (String::from("мамойклянусь"), String::from("having")),
            (String::from("выстелить по"), String::from("order by")),
            (String::from("хуйнуть"), String::from("set")),
            (String::from("посередке"), String::from("between")),
            (String::from("схуярить посередке"), String::from("inner join")),
            (String::from("схуярить слева"), String::from("left join")),
            (String::from("схуярить справа"), String::from("right join")),
            (String::from("схруярить все"), String::from("full join")),
            (String::from("схуярить"), String::from("join")),
            (String::from("табло"), String::from("table")),
            (String::from("ментяра"), String::from("index")),
            (String::from("кличка"), String::from("as")),
            (String::from("хуйня"), String::from("values")),
            (String::from("нечетко"), String::from("false")),
            (String::from("четко"), String::from("true")),
            (String::from("очко"), String::from("null")),
            (String::from("семка"), String::from("integer")),
            (String::from("короткоствол"), String::from("smallint")),
            (String::from("длинныйсериал"), String::from("bigserial")),
            (String::from("сериал"), String::from("serial")),
            (String::from("пацан"), String::from("bool")),
            (String::from("нетрушнаямалява"), String::from("char")),
            (String::from("трушнаямалява"), String::from("varchar")),
            (String::from("малява"), String::from("text")),
            (String::from("залупаб"), String::from("jsonb")),
            (String::from("залупа"), String::from("json")),
            (String::from("мойномер"), String::from("uuid")),
            (String::from("ссылочка"), String::from("references")),
            (String::from("пахан"), String::from("primary key")),
            (String::from("еслинехуй"), String::from("default")),
            (String::from("или"), String::from("or")),
            (String::from("нихуя"), String::from("not")),
            (String::from("хуй"), String::from("big")),
            (String::from("и"), String::from("and")),
            (String::from("в"), String::from("into")),
        ];

        Commands {
            dict
        }
    }

    pub fn is_exit(command: &String) -> bool {
        command == EXIT_COMMAND
    }

    pub fn compile_sql(&self, mut command: String) -> String {
        self.dict.iter().for_each(|(k, v)| {
            command = str::replace(&command, k.as_str(), v.as_str());
            command = str::replace(&command, k.to_uppercase().as_str(), v.to_uppercase().as_str());
        });
        String::from(command)
    }

    pub fn compile_yopta(&self, mut command: String) -> String {
        self.dict.iter().for_each(|(k, v)| {
            command = str::replace(&command, v.as_str(), k.as_str());
            command = str::replace(&command, v.to_uppercase().as_str(), k.to_uppercase().as_str());
        });
        String::from(command)
    }
}