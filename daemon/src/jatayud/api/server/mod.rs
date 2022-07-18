use once_cell::sync::OnceCell;
pub const GENESIS_TEXT: OnceCell<String> = OnceCell::new();

pub fn set_genesis_text(genesis_text: String) {
    GENESIS_TEXT.get_or_init(|| genesis_text);
}
