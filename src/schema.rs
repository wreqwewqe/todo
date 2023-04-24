
diesel::table! {
    users (uuid){
        uuid->Text,
        username->Text,
        password->Text,
        phone->Nullable<Text>,
    }
}

diesel::table! {
    todo_item (id){
        id->Integer,
        owner->Text,
        title->Text,
        content->Text,
        status->Bool,
        create_at->Text,
        start_time->BigInt,
    }
}