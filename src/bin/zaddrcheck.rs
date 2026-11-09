fn main() {
    let input = std::env::args().nth(1).unwrap_or_default();
    let kind = zcash_quest_tools::address_type(&input);
    println!(
        "{}",
        serde_json::json!({"input":input,"valid":kind.is_some(),"type":kind.unwrap_or("invalid")})
    );
}
