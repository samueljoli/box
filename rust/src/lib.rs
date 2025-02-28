struct Arg {
    field: String
}

fn interface(param: Arg) -> Arg {
    Arg {
        field: "value".to_string()
    }
}
