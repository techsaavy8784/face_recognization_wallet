// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int8,
        uid -> Int8,
        #[max_length = 256]
        mnemonic -> Nullable<Varchar>,
        #[max_length = 256]
        address -> Nullable<Varchar>,
        #[max_length = 256]
        token -> Nullable<Varchar>,
        feature -> Nullable<Bytea>,
    }
}
